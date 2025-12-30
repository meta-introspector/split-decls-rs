// Generated macro for impl_463 (impl)
macro_rules! Depcrate_scopeguardimpl_463 {
() => {
// Module: crate::scopeguard
// Provides: {"impl_463"}
// Dependencies: {}
impl < T , F > ScopeGuard < T , F > where F : FnMut (& mut T) , { # [inline] pub fn into_inner (guard : Self) -> T { let guard = ManuallyDrop :: new (guard) ; unsafe { let value = ptr :: read (& guard . value) ; let _ = ptr :: read (& guard . dropfn) ; value } } }
};
}
