// Generated macro for macro_7925 (macro)
macro_rules! Depcrate_needless_maybe_sizedmacro_7925 {
() => {
// Module: crate::needless_maybe_sized
// Provides: {"macro_7925"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints `?Sized` bounds applied to type parameters that cannot be unsized"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `?Sized` bound is misleading because it cannot be satisfied by an"] # [doc = " unsized type"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " // `T` cannot be unsized because `Clone` requires it to be `Sized`"] # [doc = " fn f<T: Clone + ?Sized>(t: &T) {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " fn f<T: Clone>(t: &T) {}"] # [doc = ""] # [doc = " // or choose alternative bounds for `T` so that it can be unsized"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub NEEDLESS_MAYBE_SIZED , suspicious , "a `?Sized` bound that is unusable due to a `Sized` requirement" }
};
}
