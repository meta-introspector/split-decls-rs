use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Main entry point for completion. We run completion as a two-phase process.
///
/// First, we look at the position and collect a so-called `CompletionContext`.
/// This is a somewhat messy process, because, during completion, syntax tree is
/// incomplete and can look really weird.
///
/// Once the context is collected, we run a series of completion routines which
/// look at the context and produce completion items. One subtlety about this
/// phase is that completion engine should not filter by the substring which is
/// already present, it should give all possible variants for the identifier at
/// the caret. In other words, for
///
/// ```ignore
/// fn f() {
///     let foo = 92;
///     let _ = bar$0
/// }
/// ```
///
/// `foo` *should* be present among the completion variants. Filtering by
/// identifier prefix/fuzzy match should be done higher in the stack, together
/// with ordering of completions (currently this is done by the client).
///
/// # Speculative Completion Problem
///
/// There's a curious unsolved problem in the current implementation. Often, you
/// want to compute completions on a *slightly different* text document.
///
/// In the simplest case, when the code looks like `let x = `, you want to
/// insert a fake identifier to get a better syntax tree: `let x = complete_me`.
///
/// We do this in `CompletionContext`, and it works OK-enough for *syntax*
/// analysis. However, we might want to, eg, ask for the type of `complete_me`
/// variable, and that's where our current infrastructure breaks down. salsa
/// doesn't allow such "phantom" inputs.
///
/// Another case where this would be instrumental is macro expansion. We want to
/// insert a fake ident and re-expand code. There's `expand_speculative` as a
/// workaround for this.
///
/// A different use-case is completion of injection (examples and links in doc
/// comments). When computing completion for a path in a doc-comment, you want
/// to inject a fake path expression into the item being documented and complete
/// that.
///
/// IntelliJ has CodeFragment/Context infrastructure for that. You can create a
/// temporary PSI node, and say that the context ("parent") of this node is some
/// existing node. Asking for, eg, type of this `CodeFragment` node works
/// correctly, as the underlying infrastructure makes use of contexts to do
/// analysis.
pub fn completions(
    db: &RootDatabase,
    config: &CompletionConfig<'_>,
    position: FilePosition,
    trigger_character: Option<char>,
) -> Option<Vec<CompletionItem>> {
    let (ctx, analysis) = &CompletionContext::new(
        db,
        position,
        config,
        trigger_character,
    )?;
    let mut completions = Completions::default();
    if trigger_character == Some('(') {
        if let CompletionAnalysis::NameRef(
            NameRefContext {
                kind: NameRefKind::Path(
                    path_ctx @ PathCompletionCtx {
                        kind: PathKind::Vis { has_in_token },
                        ..
                    },
                ),
                ..
            },
        ) = analysis {
            completions::vis::complete_vis_path(
                &mut completions,
                ctx,
                path_ctx,
                has_in_token,
            );
        }
        return Some(completions.into());
    }
    if trigger_character == Some('_')
        && ctx.original_token.kind() == syntax::SyntaxKind::UNDERSCORE
        && let CompletionAnalysis::NameRef(
            NameRefContext {
                kind: NameRefKind::Path(
                    path_ctx @ PathCompletionCtx {
                        kind: PathKind::Type { .. } | PathKind::Pat { .. },
                        ..
                    },
                ),
                ..
            },
        ) = analysis && path_ctx.is_trivial_path()
    {
        return None;
    }
    {
        let acc = &mut completions;
        match analysis {
            CompletionAnalysis::Name(name_ctx) => {
                completions::complete_name(acc, ctx, name_ctx)
            }
            CompletionAnalysis::NameRef(name_ref_ctx) => {
                completions::complete_name_ref(acc, ctx, name_ref_ctx)
            }
            CompletionAnalysis::Lifetime(lifetime_ctx) => {
                completions::lifetime::complete_label(acc, ctx, lifetime_ctx);
                completions::lifetime::complete_lifetime(acc, ctx, lifetime_ctx);
            }
            CompletionAnalysis::String { original, expanded: Some(expanded) } => {
                completions::extern_abi::complete_extern_abi(acc, ctx, expanded);
                completions::format_string::format_string(acc, ctx, original, expanded);
                completions::env_vars::complete_cargo_env_vars(
                    acc,
                    ctx,
                    original,
                    expanded,
                );
                completions::ra_fixture::complete_ra_fixture(
                    acc,
                    ctx,
                    original,
                    expanded,
                );
            }
            CompletionAnalysis::UnexpandedAttrTT {
                colon_prefix,
                fake_attribute_under_caret: Some(attr),
                extern_crate,
            } => {
                completions::attribute::complete_known_attribute_input(
                    acc,
                    ctx,
                    colon_prefix,
                    attr,
                    extern_crate.as_ref(),
                );
            }
            CompletionAnalysis::UnexpandedAttrTT { .. }
            | CompletionAnalysis::String { .. } => {}
        }
    }
    Some(completions.into())
}
