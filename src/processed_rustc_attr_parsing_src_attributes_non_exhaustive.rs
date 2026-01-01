// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/non_exhaustive.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::Target;
use crate::rustc_complete::attrs::AttributeKind;
use crate::rustc_complete::{Span, Symbol, sym};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::attributes::{NoArgsAttributeParser, OnDuplicate};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::context::Stage;
use crate::target_checking::AllowedTargets;
use crate::target_checking::Policy::{Allow, Warn};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=17 */

pub(crate) struct NonExhaustiveParser;

impl<S: Stage> NoArgsAttributeParser<S> for NonExhaustiveParser {
    const PATH: &[Symbol] = &[sym::non_exhaustive];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Enum),
        Allow(Target::Struct),
        Allow(Target::Variant),
        Warn(Target::Field),
        Warn(Target::Arm),
        Warn(Target::MacroDef),
        Warn(Target::MacroCall),
    ]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::NonExhaustive;
}