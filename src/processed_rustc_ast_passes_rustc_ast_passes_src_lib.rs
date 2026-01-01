// SRC: ../rust/compiler/rustc_ast_passes/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=MODULE | NAME=UNNAMED | COMPLEXITY=3 | LINES=18 */
// The `rustc_ast_passes` crate contains passes which validate the AST in `syntax`
// parsed by `rustc_parse` and then lowered, after the passes in this crate,
// by `rustc_ast_lowering`.

// tidy-alphabetical-start
#[allow(internal_features)]
#[doc(rust_logo)]
#[feature(box_patterns)]
#[feature(if_let_guard)]
#[feature(iter_is_partitioned)]
#[feature(rustdoc_internals)]
// tidy-alphabetical-end


rustc_fluent_macro::fluent_messages! { "../messages.ftl" }