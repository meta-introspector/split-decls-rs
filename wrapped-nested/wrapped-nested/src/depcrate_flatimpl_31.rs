// Generated macro for impl_31 (impl)
macro_rules! Depcrate_flatimpl_31 {
() => {
// Module: crate::flat
// Provides: {"impl_31"}
// Dependencies: {}
impl < S > fmt :: Debug for Tagged < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Tagged") . field ("tag" , & self . tag) . field ("label" , & self . label) . field ("index" , & self . index) . finish_non_exhaustive () } }
};
}
