// Generated macro for impl_197 (impl)
macro_rules! Depcrate_sequenceimpl_197 {
() => {
// Module: crate::sequence
// Provides: {"impl_197"}
// Dependencies: {}
unsafe impl < 'a , T , NM , N > Unflatten < T , NM , N > for & 'a mut GenericArray < T , NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { type Output = & 'a mut GenericArray < GenericArray < T , N > , Quot < NM , N > > ; # [inline (always)] fn unflatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
};
}
