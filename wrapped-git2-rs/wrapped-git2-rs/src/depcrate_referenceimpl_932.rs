// Generated macro for impl_932 (impl)
macro_rules! Depcrate_referenceimpl_932 {
() => {
// Module: crate::reference
// Provides: {"impl_932"}
// Dependencies: {}
impl < 'repo > Ord for Reference < 'repo > { fn cmp (& self , other : & Reference < 'repo >) -> Ordering { c_cmp_to_ordering (unsafe { raw :: git_reference_cmp (& * self . raw , & * other . raw) }) } }
};
}
