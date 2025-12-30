// Generated macro for impl_252 (impl)
macro_rules! Depcrate_module_lattice_utilimpl_252 {
() => {
// Module: crate::module_lattice::util
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'a , T , N , M > Unflatten < M > for & 'a Array < T , N > where T : Default , N : ArraySize + Div < M > + Rem < M , Output = U0 > , M : ArraySize , Quot < N , M > : ArraySize , { type Part = & 'a Array < T , Quot < N , M > > ; fn unflatten (self) -> Array < Self :: Part , M > { let part_size = Quot :: < N , M > :: USIZE ; let mut ptr : * const T = self . as_ptr () ; Array :: from_fn (| _i | unsafe { let part = & * (ptr . cast ()) ; ptr = ptr . add (part_size) ; part }) } }
};
}
