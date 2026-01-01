// SRC: ../rust/compiler/rustc_builtin_macros/src/contracts.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::token;
use crate::rustc_complete::tokenstream::{DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTree};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::ErrorGuaranteed;
use crate::rustc_expand::base::{AttrProcMacro, ExtCtxt};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::Span;
use crate::rustc_complete::symbol::{Ident, Symbol, kw};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=expand | COMPLEXITY=5 | LINES=16 */

pub(crate) struct ExpandRequires;

pub(crate) struct ExpandEnsures;

impl AttrProcMacro for ExpandRequires {
    fn expand<'cx>(
        &self,
        ecx: &'cx mut ExtCtxt<'_>,
        span: Span,
        annotation: TokenStream,
        annotated: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        expand_requires_tts(ecx, span, annotation, annotated)
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=expand | COMPLEXITY=5 | LINES=12 */

impl AttrProcMacro for ExpandEnsures {
    fn expand<'cx>(
        &self,
        ecx: &'cx mut ExtCtxt<'_>,
        span: Span,
        annotation: TokenStream,
        annotated: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        expand_ensures_tts(ecx, span, annotation, annotated)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=expand_contract_clause | COMPLEXITY=45 | LINES=89 */

/// Expand the function signature to include the contract clause.
///
/// The contracts clause will be injected before the function body and the optional where clause.
/// For that, we search for the body / where token, and invoke the `inject` callback to generate the
/// contract clause in the right place.
///
// FIXME: this kind of manual token tree munging does not have significant precedent among
// rustc builtin macros, probably because most builtin macros use direct AST manipulation to
// accomplish similar goals. But since our attributes need to take arbitrary expressions, and
// our attribute infrastructure does not yet support mixing a token-tree annotation with an AST
// annotated, we end up doing token tree manipulation.
fn expand_contract_clause(
    ecx: &mut ExtCtxt<'_>,
    attr_span: Span,
    annotated: TokenStream,
    inject: impl FnOnce(&mut TokenStream) -> Result<(), ErrorGuaranteed>,
) -> Result<TokenStream, ErrorGuaranteed> {
    let mut new_tts = TokenStream::default();
    let mut cursor = annotated.iter();

    let is_kw = |tt: &TokenTree, sym: Symbol| {
        if let TokenTree::Token(token, _) = tt { token.is_ident_named(sym) } else { false }
    };

    // Find the `fn` keyword to check if this is a function.
    if cursor
        .find(|tt| {
            new_tts.push_tree((*tt).clone());
            is_kw(tt, kw::Fn)
        })
        .is_none()
    {
        return Err(ecx
            .sess
            .dcx()
            .span_err(attr_span, "contract annotations can only be used on functions"));
    }

    // Found the `fn` keyword, now find either the `where` token or the function body.
    let next_tt = loop {
        let Some(tt) = cursor.next() else {
            return Err(ecx.sess.dcx().span_err(
                attr_span,
                "contract annotations is only supported in functions with bodies",
            ));
        };
        // If `tt` is the last element. Check if it is the function body.
        if cursor.peek().is_none() {
            if let TokenTree::Delimited(_, _, token::Delimiter::Brace, _) = tt {
                break tt;
            } else {
                return Err(ecx.sess.dcx().span_err(
                    attr_span,
                    "contract annotations is only supported in functions with bodies",
                ));
            }
        }

        if is_kw(tt, kw::Where) {
            break tt;
        }
        new_tts.push_tree(tt.clone());
    };

    // At this point, we've transcribed everything from the `fn` through the formal parameter list
    // and return type declaration, (if any), but `tt` itself has *not* been transcribed.
    //
    // Now inject the AST contract form.
    //
    inject(&mut new_tts)?;

    // Above we injected the internal AST requires/ensures construct. Now copy over all the other
    // token trees.
    new_tts.push_tree(next_tt.clone());
    while let Some(tt) = cursor.next() {
        new_tts.push_tree(tt.clone());
        if cursor.peek().is_none()
            && !matches!(tt, TokenTree::Delimited(_, _, token::Delimiter::Brace, _))
        {
            return Err(ecx.sess.dcx().span_err(
                attr_span,
                "contract annotations is only supported in functions with bodies",
            ));
        }
    }

    Ok(new_tts)
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=expand_requires_tts | COMPLEXITY=4 | LINES=26 */

fn expand_requires_tts(
    ecx: &mut ExtCtxt<'_>,
    attr_span: Span,
    annotation: TokenStream,
    annotated: TokenStream,
) -> Result<TokenStream, ErrorGuaranteed> {
    let feature_span = ecx.with_def_site_ctxt(attr_span);
    expand_contract_clause(ecx, attr_span, annotated, |new_tts| {
        new_tts.push_tree(TokenTree::Token(
            token::Token::from_ast_ident(Ident::new(kw::ContractRequires, feature_span)),
            Spacing::Joint,
        ));
        new_tts.push_tree(TokenTree::Token(
            token::Token::new(token::TokenKind::OrOr, attr_span),
            Spacing::Alone,
        ));
        new_tts.push_tree(TokenTree::Delimited(
            DelimSpan::from_single(attr_span),
            DelimSpacing::new(Spacing::JointHidden, Spacing::JointHidden),
            token::Delimiter::Parenthesis,
            annotation,
        ));
        Ok(())
    })
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=expand_ensures_tts | COMPLEXITY=4 | LINES=22 */

fn expand_ensures_tts(
    ecx: &mut ExtCtxt<'_>,
    attr_span: Span,
    annotation: TokenStream,
    annotated: TokenStream,
) -> Result<TokenStream, ErrorGuaranteed> {
    let feature_span = ecx.with_def_site_ctxt(attr_span);
    expand_contract_clause(ecx, attr_span, annotated, |new_tts| {
        new_tts.push_tree(TokenTree::Token(
            token::Token::from_ast_ident(Ident::new(kw::ContractEnsures, feature_span)),
            Spacing::Joint,
        ));
        new_tts.push_tree(TokenTree::Delimited(
            DelimSpan::from_single(attr_span),
            DelimSpacing::new(Spacing::JointHidden, Spacing::JointHidden),
            token::Delimiter::Parenthesis,
            annotation,
        ));
        Ok(())
    })
}