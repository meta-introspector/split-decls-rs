// Generated macro for macro_1101 (macro)
macro_rules! Depcrate_castsmacro_1101 {
() => {
// Module: crate::casts
// Provides: {"macro_1101"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of `as *const _` or `as *mut _` conversion using inferred type."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The conversion might include a dangerous cast that might go undetected due to the type being inferred."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn as_usize<T>(t: &T) -> usize {"] # [doc = "     // BUG: `t` is already a reference, so we will here"] # [doc = "     // return a dangling pointer to a temporary value instead"] # [doc = "     &t as *const _ as usize"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn as_usize<T>(t: &T) -> usize {"] # [doc = "     t as *const T as usize"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.85.0"] pub AS_POINTER_UNDERSCORE , restriction , "detects `as *mut _` and `as *const _` conversion" }
};
}
