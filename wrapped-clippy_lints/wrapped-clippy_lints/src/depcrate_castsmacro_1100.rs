// Generated macro for macro_1100 (macro)
macro_rules! Depcrate_castsmacro_1100 {
() => {
// Module: crate::casts
// Provides: {"macro_1100"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts of references to pointer using `as`"] # [doc = " and suggests `std::ptr::from_ref` and `std::ptr::from_mut` instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `as` casts may result in silently changing mutability or type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a_ref = &1;"] # [doc = " let a_ptr = a_ref as *const _;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a_ref = &1;"] # [doc = " let a_ptr = std::ptr::from_ref(a_ref);"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub REF_AS_PTR , pedantic , "using `as` to cast a reference to pointer" }
};
}
