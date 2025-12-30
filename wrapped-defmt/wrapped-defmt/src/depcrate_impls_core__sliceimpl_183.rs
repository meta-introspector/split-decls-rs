// Generated macro for impl_183 (impl)
macro_rules! Depcrate_impls_core__sliceimpl_183 {
() => {
// Module: crate::impls::core_::slice
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'a , T : 'a > Format for core :: slice :: Iter < 'a , T > where T : Format , { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "Iter {{ slice: {=[?]}, position: ? }}" , self . as_slice ()) } }
};
}
