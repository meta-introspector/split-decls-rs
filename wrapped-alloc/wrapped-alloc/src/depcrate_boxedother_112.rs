// Generated macro for other_112 (other)
macro_rules! Depcrate_boxedother_112 {
() => {
// Module: crate::boxed
// Provides: {"other_112"}
// Dependencies: {}
# [doc = " Constructs a `Box<T>` by calling the `exchange_malloc` lang item and moving the argument into"] # [doc = " the newly allocated memory. This is an intrinsic to avoid unnecessary copies."] # [doc = ""] # [doc = " This is the surface syntax for `box <expr>` expressions."] # [rustc_intrinsic] # [unstable (feature = "liballoc_internals" , issue = "none")] pub fn box_new < T > (x : T) -> Box < T >;
};
}
