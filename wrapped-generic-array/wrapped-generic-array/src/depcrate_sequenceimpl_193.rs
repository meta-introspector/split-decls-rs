// Generated macro for impl_193 (impl)
macro_rules! Depcrate_sequenceimpl_193 {
() => {
// Module: crate::sequence
// Provides: {"impl_193"}
// Dependencies: {}
unsafe impl < 'a , T , N , M > Flatten < T , N , M > for & 'a GenericArray < GenericArray < T , N > , M > where N : ArrayLength + Mul < M > , M : ArrayLength , Prod < N , M > : ArrayLength , { type Output = & 'a GenericArray < T , Prod < N , M > > ; # [inline (always)] fn flatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
};
}
