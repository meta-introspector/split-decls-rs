// Generated macro for macro_7583 (macro)
macro_rules! Depcrate_mutable_debug_assertionmacro_7583 {
() => {
// Module: crate::mutable_debug_assertion
// Provides: {"macro_7583"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for function/method calls with a mutable"] # [doc = " parameter in `debug_assert!`, `debug_assert_eq!` and `debug_assert_ne!` macros."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In release builds `debug_assert!` macros are optimized out by the"] # [doc = " compiler."] # [doc = " Therefore mutating something in a `debug_assert!` macro results in different behavior"] # [doc = " between a release and debug build."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " debug_assert_eq!(vec![3].pop(), Some(3));"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " # let mut x = 5;"] # [doc = " # fn takes_a_mut_parameter(_: &mut u32) -> bool { unimplemented!() }"] # [doc = " debug_assert!(takes_a_mut_parameter(&mut x));"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub DEBUG_ASSERT_WITH_MUT_CALL , nursery , "mutable arguments in `debug_assert{,_ne,_eq}!`" }
};
}
