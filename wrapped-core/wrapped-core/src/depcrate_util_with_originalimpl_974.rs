// Generated macro for impl_974 (impl)
macro_rules! Depcrate_util_with_originalimpl_974 {
() => {
// Module: crate::util::with_original
// Provides: {"impl_974"}
// Dependencies: {}
# [doc = " Get the lifetime usage of `parsed`."] impl < P : UsesLifetimes , O > UsesLifetimes for WithOriginal < P , O > { fn uses_lifetimes < 'a > (& self , options : & crate :: usage :: Options , lifetimes : & 'a crate :: usage :: LifetimeSet ,) -> crate :: usage :: LifetimeRefSet < 'a > { self . parsed . uses_lifetimes (options , lifetimes) } }
};
}
