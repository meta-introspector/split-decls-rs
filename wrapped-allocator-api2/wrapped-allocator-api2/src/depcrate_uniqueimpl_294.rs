// Generated macro for impl_294 (impl)
macro_rules! Depcrate_uniqueimpl_294 {
() => {
// Module: crate::unique
// Provides: {"impl_294"}
// Dependencies: {}
# [doc = " `Unique` pointers are `Sync` if `T` is `Sync` because the data they"] # [doc = " reference is unaliased. Note that this aliasing invariant is"] # [doc = " unenforced by the type system; the abstraction using the"] # [doc = " `Unique` must enforce it."] unsafe impl < T : Sync + ? Sized > Sync for Unique < T > { }
};
}
