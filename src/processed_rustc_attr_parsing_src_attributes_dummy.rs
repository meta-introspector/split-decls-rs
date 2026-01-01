// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/dummy.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_feature::{AttributeTemplate, template};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::attrs::AttributeKind;
use crate::rustc_complete::{Symbol, sym};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::attributes::{AttributeOrder, OnDuplicate, SingleAttributeParser};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::context::{AcceptContext, Stage};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::parser::ArgParser;
use crate::target_checking::{ALL_TARGETS, AllowedTargets};
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=convert | COMPLEXITY=6 | LINES=13 */

pub(crate) struct DummyParser;
impl<S: Stage> SingleAttributeParser<S> for DummyParser {
    const PATH: &[Symbol] = &[sym::rustc_dummy];
    const ATTRIBUTE_ORDER: AttributeOrder = AttributeOrder::KeepInnermost;
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Ignore;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(ALL_TARGETS);
    const TEMPLATE: AttributeTemplate = template!(Word); // Anything, really

    fn convert(_: &mut AcceptContext<'_, '_, S>, _: &ArgParser<'_>) -> Option<AttributeKind> {
        Some(AttributeKind::Dummy)
    }
}