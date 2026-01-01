// SRC: ../rust/compiler/rustc_session/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=15 */
// tidy-alphabetical-start
#[allow(internal_features)]
#[cfg_attr(bootstrap, feature(round_char_boundary))]
#[feature(default_field_values)]
#[feature(iter_intersperse)]
#[feature(rustc_attrs)]
// To generate CodegenOptionsTargetModifiers and UnstableOptionsTargetModifiers enums
// with macro_rules, it is necessary to use recursive mechanic ("Incremental TT Munchers").
#[recursion_limit = "256"]
// tidy-alphabetical-end


pub use lint::{declare_lint, declare_lint_pass, declare_tool_lint, impl_lint_pass};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=19 */
pub use rustc_lint_defs as lint;

#[macro_use]

pub use session::*;


pub use getopts;

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=5 */

/// Requirements for a `StableHashingContext` to be used in this crate.
/// This is a hack to allow using the `HashStable_Generic` derive macro
/// instead of implementing everything in `rustc_middle`.
pub trait HashStableContext: crate::rustc_ast::HashStableContext + crate::rustc_hir::HashStableContext {}