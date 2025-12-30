// Generated macro for impl_47 (impl)
macro_rules! Depcrate_typesimpl_47 {
() => {
// Module: crate::types
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > fmt :: Display for BytesOrWideString < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . to_str_lossy () . fmt (f) } }
};
}
