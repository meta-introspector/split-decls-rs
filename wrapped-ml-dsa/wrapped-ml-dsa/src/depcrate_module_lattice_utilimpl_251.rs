// Generated macro for impl_251 (impl)
macro_rules! Depcrate_module_lattice_utilimpl_251 {
() => {
// Module: crate::module_lattice::util
// Provides: {"impl_251"}
// Dependencies: {}
impl < T , N , M > Unflatten < M > for Array < T , N > where T : Default , N : ArraySize + Div < M > + Rem < M , Output = U0 > , M : ArraySize , Quot < N , M > : ArraySize , { type Part = Array < T , Quot < N , M > > ; fn unflatten (self) -> Array < Self :: Part , M > { let part_size = Quot :: < N , M > :: USIZE ; let whole = ManuallyDrop :: new (self) ; Array :: from_fn (| i | unsafe { ptr :: read (whole . as_ptr () . add (i * part_size) . cast ()) }) } }
};
}
