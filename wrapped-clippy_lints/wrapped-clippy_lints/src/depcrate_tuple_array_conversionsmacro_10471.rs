// Generated macro for macro_10471 (macro)
macro_rules! Depcrate_tuple_array_conversionsmacro_10471 {
() => {
// Module: crate::tuple_array_conversions
// Provides: {"macro_10471"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for tuple<=>array conversions that are not done with `.into()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It may be unnecessary complexity. `.into()` works for converting tuples<=> arrays of up to"] # [doc = " 12 elements and conveys the intent more clearly, while also leaving less room for hard to"] # [doc = " spot bugs!"] # [doc = ""] # [doc = " ### Known issues"] # [doc = " The suggested code may hide potential asymmetry in some cases. See"] # [doc = " [#11085](https://github.com/rust-lang/rust-clippy/issues/11085) for more info."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let t1 = &[(1, 2), (3, 4)];"] # [doc = " let v1: Vec<[u32; 2]> = t1.iter().map(|&(a, b)| [a, b]).collect();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let t1 = &[(1, 2), (3, 4)];"] # [doc = " let v1: Vec<[u32; 2]> = t1.iter().map(|&t| t.into()).collect();"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub TUPLE_ARRAY_CONVERSIONS , nursery , "checks for tuple<=>array conversions that are not done with `.into()`" }
};
}
