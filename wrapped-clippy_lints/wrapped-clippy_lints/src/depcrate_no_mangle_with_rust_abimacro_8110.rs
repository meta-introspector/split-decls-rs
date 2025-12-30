// Generated macro for macro_8110 (macro)
macro_rules! Depcrate_no_mangle_with_rust_abimacro_8110 {
() => {
// Module: crate::no_mangle_with_rust_abi
// Provides: {"macro_8110"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for Rust ABI functions with the `#[no_mangle]` attribute."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The Rust ABI is not stable, but in many simple cases matches"] # [doc = " enough with the C ABI that it is possible to forget to add"] # [doc = " `extern \"C\"` to a function called from C. Changes to the"] # [doc = " Rust ABI can break this at any point."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = "  #[no_mangle]"] # [doc = "  fn example(arg_one: u32, arg_two: usize) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = "  #[no_mangle]"] # [doc = "  extern \"C\" fn example(arg_one: u32, arg_two: usize) {}"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub NO_MANGLE_WITH_RUST_ABI , pedantic , "convert Rust ABI functions to C ABI" }
};
}
