// Generated macro for impl_205 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_205 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_205"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Sub < & NttVector < F , K > > for & NttVector < F , K > { type Output = NttVector < F , K > ; fn sub (self , rhs : & NttVector < F , K >) -> NttVector < F , K > { NttVector (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (x , y) | x - y) . collect () ,) } }
};
}
