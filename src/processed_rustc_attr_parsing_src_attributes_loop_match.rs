// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/loop_match.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=9 */
use crate::prelude::*;

pub(crate) struct LoopMatchParser;
impl<S: Stage> NoArgsAttributeParser<S> for LoopMatchParser {
    const PATH: &[Symbol] = &[sym::loop_match];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Expression)]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::LoopMatch;
}
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=8 */

pub(crate) struct ConstContinueParser;
impl<S: Stage> NoArgsAttributeParser<S> for ConstContinueParser {
    const PATH: &[Symbol] = &[sym::const_continue];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Expression)]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::ConstContinue;
}