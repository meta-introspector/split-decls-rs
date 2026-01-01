// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/prototype.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
// Attributes that are only used on function prototypes.

use crate::rustc_feature::{AttributeTemplate, template};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::Target;
use crate::rustc_complete::attrs::{AttributeKind, MirDialect, MirPhase};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Span, Symbol, sym};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use super::{AttributeOrder, OnDuplicate};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::attributes::SingleAttributeParser;
use crate::context::{AcceptContext, Stage};
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=convert | COMPLEXITY=27 | LINES=58 */
use crate::parser::ArgParser;
use crate::target_checking::AllowedTargets;
use crate::target_checking::Policy::Allow;

pub(crate) struct CustomMirParser;

impl<S: Stage> SingleAttributeParser<S> for CustomMirParser {
    const PATH: &[crate::rustc_span::Symbol] = &[sym::custom_mir];

    const ATTRIBUTE_ORDER: AttributeOrder = AttributeOrder::KeepOutermost;

    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;

    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Fn)]);

    const TEMPLATE: AttributeTemplate = template!(List: &[r#"dialect = "...", phase = "...""#]);

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser<'_>) -> Option<AttributeKind> {
        let Some(list) = args.list() else {
            cx.expected_list(cx.attr_span);
            return None;
        };

        let mut dialect = None;
        let mut phase = None;
        let mut failed = false;

        for item in list.mixed() {
            let Some(meta_item) = item.meta_item() else {
                cx.expected_name_value(item.span(), None);
                failed = true;
                break;
            };

            if let Some(arg) = meta_item.word_is(sym::dialect) {
                extract_value(cx, sym::dialect, arg, meta_item.span(), &mut dialect, &mut failed);
            } else if let Some(arg) = meta_item.word_is(sym::phase) {
                extract_value(cx, sym::phase, arg, meta_item.span(), &mut phase, &mut failed);
            } else if let Some(word) = meta_item.path().word() {
                let word = word.to_string();
                cx.unknown_key(meta_item.span(), word, &["dialect", "phase"]);
                failed = true;
            } else {
                cx.expected_name_value(meta_item.span(), None);
                failed = true;
            };
        }

        let dialect = parse_dialect(cx, dialect, &mut failed);
        let phase = parse_phase(cx, phase, &mut failed);

        if failed {
            return None;
        }

        Some(AttributeKind::CustomMir(dialect, phase, cx.attr_span))
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=extract_value | COMPLEXITY=8 | LINES=29 */

fn extract_value<S: Stage>(
    cx: &mut AcceptContext<'_, '_, S>,
    key: Symbol,
    arg: &ArgParser<'_>,
    span: Span,
    out_val: &mut Option<(Symbol, Span)>,
    failed: &mut bool,
) {
    if out_val.is_some() {
        cx.duplicate_key(span, key);
        *failed = true;
        return;
    }

    let Some(val) = arg.name_value() else {
        cx.expected_single_argument(arg.span().unwrap_or(span));
        *failed = true;
        return;
    };

    let Some(value_sym) = val.value_as_str() else {
        cx.expected_string_literal(val.value_span, Some(val.value_as_lit()));
        *failed = true;
        return;
    };

    *out_val = Some((value_sym, val.value_span));
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=parse_dialect | COMPLEXITY=8 | LINES=22 */

fn parse_dialect<S: Stage>(
    cx: &mut AcceptContext<'_, '_, S>,
    dialect: Option<(Symbol, Span)>,
    failed: &mut bool,
) -> Option<(MirDialect, Span)> {
    let (dialect, span) = dialect?;

    let dialect = match dialect {
        sym::analysis => MirDialect::Analysis,
        sym::built => MirDialect::Built,
        sym::runtime => MirDialect::Runtime,

        _ => {
            cx.expected_specific_argument(span, &[sym::analysis, sym::built, sym::runtime]);
            *failed = true;
            return None;
        }
    };

    Some((dialect, span))
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=parse_phase | COMPLEXITY=8 | LINES=22 */

fn parse_phase<S: Stage>(
    cx: &mut AcceptContext<'_, '_, S>,
    phase: Option<(Symbol, Span)>,
    failed: &mut bool,
) -> Option<(MirPhase, Span)> {
    let (phase, span) = phase?;

    let phase = match phase {
        sym::initial => MirPhase::Initial,
        sym::post_cleanup => MirPhase::PostCleanup,
        sym::optimized => MirPhase::Optimized,

        _ => {
            cx.expected_specific_argument(span, &[sym::initial, sym::post_cleanup, sym::optimized]);
            *failed = true;
            return None;
        }
    };

    Some((phase, span))
}