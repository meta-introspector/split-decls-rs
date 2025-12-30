// Generated macro for macro_2157 (macro)
macro_rules! Depcrate_extra_unused_type_parametersmacro_2157 {
() => {
// Module: crate::extra_unused_type_parameters
// Provides: {"macro_2157"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for type parameters in generics that are never used anywhere else."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Functions cannot infer the value of unused type parameters; therefore, calling them"] # [doc = " requires using a turbofish, which serves no purpose but to satisfy the compiler."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn unused_ty<T>(x: u8) {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn no_unused_ty(x: u8) {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub EXTRA_UNUSED_TYPE_PARAMETERS , complexity , "unused type parameters in function definitions" }
};
}
