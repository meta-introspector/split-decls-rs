// Generated macro for impl_192 (impl)
macro_rules! Depcrate_sequenceimpl_192 {
() => {
// Module: crate::sequence
// Provides: {"impl_192"}
// Dependencies: {}
unsafe impl < T , N , M > Flatten < T , N , M > for GenericArray < GenericArray < T , N > , M > where N : ArrayLength + Mul < M > , M : ArrayLength , Prod < N , M > : ArrayLength , { type Output = GenericArray < T , Prod < N , M > > ; # [inline (always)] fn flatten (self) -> Self :: Output { unsafe { crate :: const_transmute (self) } } }
};
}
