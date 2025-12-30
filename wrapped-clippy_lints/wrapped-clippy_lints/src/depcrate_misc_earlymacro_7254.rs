// Generated macro for macro_7254 (macro)
macro_rules! Depcrate_misc_earlymacro_7254 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7254"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if a generic shadows a built-in type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This gives surprising type errors."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```ignore"] # [doc = " impl<u32> Foo<u32> {"] # [doc = "     fn impl_func(&self) -> u32 {"] # [doc = "         42"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub BUILTIN_TYPE_SHADOW , style , "shadowing a builtin type" }
};
}
