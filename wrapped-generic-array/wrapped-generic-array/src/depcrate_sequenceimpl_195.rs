// Generated macro for impl_195 (impl)
macro_rules! Depcrate_sequenceimpl_195 {
() => {
// Module: crate::sequence
// Provides: {"impl_195"}
// Dependencies: {}
unsafe impl < T , NM , N > Unflatten < T , NM , N > for GenericArray < T , NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { type Output = GenericArray < GenericArray < T , N > , Quot < NM , N > > ; # [inline (always)] fn unflatten (self) -> Self :: Output { unsafe { crate :: const_transmute (self) } } }
};
}
