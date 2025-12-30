// Generated macro for impl_204 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_204 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_204"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Add < & NttVector < F , K > > for & NttVector < F , K > { type Output = NttVector < F , K > ; fn add (self , rhs : & NttVector < F , K >) -> NttVector < F , K > { NttVector (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (x , y) | x + y) . collect () ,) } }
};
}
