// SRC: ../rust/compiler/rustc_hir/src/arena.rs
/* AST_META: AST_ID=1 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=14 */
/// This higher-order macro declares a list of types which can be allocated by `Arena`.
/// Note that all `Copy` types can be allocated by default and need not be specified here.
#[macro_export]
macro_rules! hir_arena_types {
    ($macro:path) => (
        $macro!([
            // HIR types
            [] asm_template: crate::rustc_ast::InlineAsmTemplatePiece,
            [] attribute: crate::rustc_hir::Attribute,
            [] owner_info: crate::rustc_hir::OwnerInfo<'tcx>,
            [] macro_def: crate::rustc_ast::MacroDef,
        ]);
    )
}