// Generated macro for impl_249 (impl)
macro_rules! Depcrate_module_lattice_utilimpl_249 {
() => {
// Module: crate::module_lattice::util
// Provides: {"impl_249"}
// Dependencies: {}
impl < T , N , M > Flatten < T , Prod < M , N > > for Array < Array < T , M > , N > where N : ArraySize , M : ArraySize + Mul < N > , Prod < M , N > : ArraySize , { type OutputSize = Prod < M , N > ; fn flatten (self) -> Array < T , Self :: OutputSize > { let whole = ManuallyDrop :: new (self) ; unsafe { ptr :: read (whole . as_ptr () . cast ()) } } }
};
}
