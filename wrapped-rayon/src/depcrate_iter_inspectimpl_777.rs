// Generated macro for impl_777 (impl)
macro_rules! Depcrate_iter_inspectimpl_777 {
() => {
// Module: crate::iter::inspect
// Provides: {"impl_777"}
// Dependencies: {}
impl < I : Debug , F > Debug for Inspect < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Inspect") . field ("base" , & self . base) . finish () } }
};
}
