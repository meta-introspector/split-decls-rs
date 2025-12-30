// Generated macro for impl_151 (impl)
macro_rules! Depcrate_imp_core_atomicimpl_151 {
() => {
// Module: crate::imp::core_atomic
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (portable_atomic_no_atomic_cas)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (target_has_atomic = "ptr"))] impl < T > AtomicPtr < T > { # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub (crate) fn compare_exchange (& self , current : * mut T , new : * mut T , success : Ordering , failure : Ordering ,) -> Result < * mut T , * mut T > { crate :: utils :: assert_compare_exchange_ordering (success , failure) ; # [cfg (portable_atomic_no_stronger_failure_ordering)] let success = crate :: utils :: upgrade_success_ordering (success , failure) ; self . inner . compare_exchange (current , new , success , failure) } # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub (crate) fn compare_exchange_weak (& self , current : * mut T , new : * mut T , success : Ordering , failure : Ordering ,) -> Result < * mut T , * mut T > { crate :: utils :: assert_compare_exchange_ordering (success , failure) ; # [cfg (portable_atomic_no_stronger_failure_ordering)] let success = crate :: utils :: upgrade_success_ordering (success , failure) ; self . inner . compare_exchange_weak (current , new , success , failure) } }
};
}
