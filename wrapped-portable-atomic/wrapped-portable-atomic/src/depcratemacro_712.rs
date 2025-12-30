// Generated macro for macro_712 (macro)
macro_rules! Depcratemacro_712 {
() => {
// Module: crate
// Provides: {"macro_712"}
// Dependencies: {}
cfg_has_atomic_128 ! { atomic_int ! (AtomicI128 , i128 , 16 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; atomic_int ! (AtomicU128 , u128 , 16 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32 , # [cfg (all (feature = "float" , portable_atomic_unstable_f128))] AtomicF128 , f128) ; }
};
}
