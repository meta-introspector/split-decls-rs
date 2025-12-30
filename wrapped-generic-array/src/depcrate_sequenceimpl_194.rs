// Generated macro for impl_194 (impl)
macro_rules! Depcrate_sequenceimpl_194 {
() => {
// Module: crate::sequence
// Provides: {"impl_194"}
// Dependencies: {}
unsafe impl < 'a , T , N , M > Flatten < T , N , M > for & 'a mut GenericArray < GenericArray < T , N > , M > where N : ArrayLength + Mul < M > , M : ArrayLength , Prod < N , M > : ArrayLength , { type Output = & 'a mut GenericArray < T , Prod < N , M > > ; # [inline (always)] fn flatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
};
}
