// Generated macro for macro_711 (macro)
macro_rules! Depcratemacro_711 {
() => {
// Module: crate
// Provides: {"macro_711"}
// Dependencies: {}
cfg_has_atomic_64 ! { atomic_int ! (AtomicI64 , i64 , 8 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; atomic_int ! (AtomicU64 , u64 , 8 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32 , # [cfg (feature = "float")] AtomicF64 , f64) ; }
};
}
