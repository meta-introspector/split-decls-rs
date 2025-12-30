// Generated macro for macro_709 (macro)
macro_rules! Depcratemacro_709 {
() => {
// Module: crate
// Provides: {"macro_709"}
// Dependencies: {}
cfg_has_atomic_16 ! { atomic_int ! (AtomicI16 , i16 , 2 , cfg_has_atomic_cas_or_amo8 , cfg_no_atomic_cas_or_amo8) ; atomic_int ! (AtomicU16 , u16 , 2 , cfg_has_atomic_cas_or_amo8 , cfg_no_atomic_cas_or_amo8 , # [cfg (all (feature = "float" , portable_atomic_unstable_f16))] AtomicF16 , f16) ; }
};
}
