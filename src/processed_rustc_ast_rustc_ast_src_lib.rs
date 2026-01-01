// SRC: ../rust/compiler/rustc_ast/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=MODULE | NAME=UNNAMED | COMPLEXITY=3 | LINES=30 */
// The Rust Abstract Syntax Tree (AST).
//
// # Note
//
// This API is completely unstable and subject to change.

// tidy-alphabetical-start
#[allow(internal_features)]
#[doc(
    html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/",
    test(attr(deny(warnings)))
)]
#[doc(rust_logo)]
#[feature(array_windows)]
#[feature(associated_type_defaults)]
#[feature(box_patterns)]
#[feature(if_let_guard)]
#[feature(macro_metavar_expr)]
#[feature(rustdoc_internals)]
#[recursion_limit = "256"]
// tidy-alphabetical-end

pub mod util {
}
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=15 */


pub use self::ast::*;
pub use self::ast_traits::{AstNodeWrapper, HasAttrs, HasNodeId, HasTokens};
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=5 */

/// Requirements for a `StableHashingContext` to be used in this crate.
/// This is a hack to allow using the `HashStable_Generic` derive macro
/// instead of implementing everything in `rustc_middle`.
pub trait HashStableContext: crate::rustc_span::HashStableContext {}