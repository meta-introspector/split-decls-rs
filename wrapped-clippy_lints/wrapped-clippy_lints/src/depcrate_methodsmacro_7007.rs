// Generated macro for macro_7007 (macro)
macro_rules! Depcrate_methodsmacro_7007 {
() => {
// Module: crate::methods
// Provides: {"macro_7007"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `MaybeUninit::uninit().assume_init()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " For most types, this is undefined behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " For now, we accept empty tuples and tuples / arrays"] # [doc = " of `MaybeUninit`. There may be other types that allow uninitialized"] # [doc = " data, but those are not yet rigorously defined."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // Beware the UB"] # [doc = " use std::mem::MaybeUninit;"] # [doc = ""] # [doc = " let _: usize = unsafe { MaybeUninit::uninit().assume_init() };"] # [doc = " ```"] # [doc = ""] # [doc = " Note that the following is OK:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::mem::MaybeUninit;"] # [doc = ""] # [doc = " let _: [MaybeUninit<bool>; 5] = unsafe {"] # [doc = "     MaybeUninit::uninit().assume_init()"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "1.39.0"] pub UNINIT_ASSUMED_INIT , correctness , "`MaybeUninit::uninit().assume_init()`" }
};
}
