// Generated macro for macro_710 (macro)
macro_rules! Depcratemacro_710 {
() => {
// Module: crate
// Provides: {"macro_710"}
// Dependencies: {}
cfg_has_atomic_32 ! { atomic_int ! (AtomicI32 , i32 , 4 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; atomic_int ! (AtomicU32 , u32 , 4 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32 , # [cfg (feature = "float")] AtomicF32 , f32) ; }
};
}
