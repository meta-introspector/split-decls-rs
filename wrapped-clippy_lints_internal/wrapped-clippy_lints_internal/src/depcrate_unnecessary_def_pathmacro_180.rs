// Generated macro for macro_180 (macro)
macro_rules! Depcrate_unnecessary_def_pathmacro_180 {
() => {
// Module: crate::unnecessary_def_path
// Provides: {"macro_180"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of def paths when a diagnostic item or a `LangItem` could be used."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The path for an item is subject to change and is less efficient to look up than a"] # [doc = " diagnostic item or a `LangItem`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " pub static VEC: PathLookup = path!(alloc::vec::Vec);"] # [doc = ""] # [doc = " VEC.contains_ty(cx, ty)"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " ty.is_diag_item(cx, sym::Vec)"] # [doc = " ```"] pub clippy :: UNNECESSARY_DEF_PATH , Warn , "using a def path when a diagnostic item or a `LangItem` is available" , report_in_external_macro : true }
};
}
