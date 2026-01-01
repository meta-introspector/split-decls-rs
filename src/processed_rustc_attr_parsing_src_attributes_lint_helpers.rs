// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/lint_helpers.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=7 | LINES=15 */
use crate::prelude::*;

pub(crate) struct AsPtrParser;
impl<S: Stage> NoArgsAttributeParser<S> for AsPtrParser {
    const PATH: &[Symbol] = &[sym::rustc_as_ptr];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Fn),
        Allow(Target::Method(MethodKind::Inherent)),
        Allow(Target::Method(MethodKind::Trait { body: false })),
        Allow(Target::Method(MethodKind::Trait { body: true })),
        Allow(Target::Method(MethodKind::TraitImpl)),
    ]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::AsPtr;
}
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=12 */

pub(crate) struct PubTransparentParser;
impl<S: Stage> NoArgsAttributeParser<S> for PubTransparentParser {
    const PATH: &[Symbol] = &[sym::rustc_pub_transparent];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Struct),
        Allow(Target::Enum),
        Allow(Target::Union),
    ]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::PubTransparent;
}
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=12 */

pub(crate) struct PassByValueParser;
impl<S: Stage> NoArgsAttributeParser<S> for PassByValueParser {
    const PATH: &[Symbol] = &[sym::rustc_pass_by_value];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Struct),
        Allow(Target::Enum),
        Allow(Target::TyAlias),
    ]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::PassByValue;
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=12 */

pub(crate) struct AutomaticallyDerivedParser;
impl<S: Stage> NoArgsAttributeParser<S> for AutomaticallyDerivedParser {
    const PATH: &[Symbol] = &[sym::automatically_derived];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowListWarnRest(&[
        Allow(Target::Impl { of_trait: true }),
        Error(Target::Crate),
        Error(Target::WherePredicate),
    ]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::AutomaticallyDerived;
}