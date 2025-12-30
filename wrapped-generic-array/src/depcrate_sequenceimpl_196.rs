// Generated macro for impl_196 (impl)
macro_rules! Depcrate_sequenceimpl_196 {
() => {
// Module: crate::sequence
// Provides: {"impl_196"}
// Dependencies: {}
unsafe impl < 'a , T , NM , N > Unflatten < T , NM , N > for & 'a GenericArray < T , NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { type Output = & 'a GenericArray < GenericArray < T , N > , Quot < NM , N > > ; # [inline (always)] fn unflatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
};
}
