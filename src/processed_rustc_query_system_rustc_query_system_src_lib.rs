// SRC: ../rust/compiler/rustc_query_system/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=15 */
// tidy-alphabetical-start
#[allow(internal_features)]
#[feature(assert_matches)]
#[feature(core_intrinsics)]
#[feature(min_specialization)]
// tidy-alphabetical-end


pub use error::{HandleCycleError, QueryOverflow, QueryOverflowNote};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
pub use values::Value;

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }