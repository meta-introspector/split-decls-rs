// Generated macro for impl_293 (impl)
macro_rules! Depcrate_uniqueimpl_293 {
() => {
// Module: crate::unique
// Provides: {"impl_293"}
// Dependencies: {}
# [doc = " `Unique` pointers are `Send` if `T` is `Send` because the data they"] # [doc = " reference is unaliased. Note that this aliasing invariant is"] # [doc = " unenforced by the type system; the abstraction using the"] # [doc = " `Unique` must enforce it."] unsafe impl < T : Send + ? Sized > Send for Unique < T > { }
};
}
