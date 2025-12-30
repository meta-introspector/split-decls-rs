// Generated macro for macro_7619 (macro)
macro_rules! Depcrate_module_stylemacro_7619 {
() => {
// Module: crate::module_style
// Provides: {"macro_7619"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks that module layout uses only `mod.rs` files."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Having multiple module layout styles in a project can be confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```text"] # [doc = " src/"] # [doc = "   stuff/"] # [doc = "     stuff_files.rs"] # [doc = "   stuff.rs"] # [doc = "   lib.rs"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```text"] # [doc = " src/"] # [doc = "   stuff/"] # [doc = "     stuff_files.rs"] # [doc = "     mod.rs"] # [doc = "   lib.rs"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub SELF_NAMED_MODULE_FILES , restriction , "checks that module layout is consistent" }
};
}
