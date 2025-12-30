// Generated macro for impl_25 (impl)
macro_rules! Depcrate_flatimpl_25 {
() => {
// Module: crate::flat
// Provides: {"impl_25"}
// Dependencies: {}
impl < S > fmt :: Debug for Map < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Map") . field ("is_key" , & self . is_key) . finish_non_exhaustive () } }
};
}
