// Generated macro for impl_780 (impl)
macro_rules! Depcrate_util_lazyimpl_780 {
() => {
// Module: crate::util::lazy
// Provides: {"impl_780"}
// Dependencies: {}
impl < T , F > Lazy < T , F > { # [doc = " Create a new `Lazy` value that is initialized via the given function."] # [doc = ""] # [doc = " The `T` type is automatically inferred from the return type of the"] # [doc = " `create` function given."] pub const fn new (create : F) -> Lazy < T , F > { Lazy (lazy :: Lazy :: new (create)) } }
};
}
