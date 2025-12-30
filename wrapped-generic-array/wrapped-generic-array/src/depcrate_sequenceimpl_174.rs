// Generated macro for impl_174 (impl)
macro_rules! Depcrate_sequenceimpl_174 {
() => {
// Module: crate::sequence
// Provides: {"impl_174"}
// Dependencies: {}
unsafe impl < 'a , T : 'a , S : GenericSequence < T > > GenericSequence < T > for & 'a S where & 'a S : IntoIterator , { type Length = S :: Length ; type Sequence = S :: Sequence ; # [inline (always)] fn generate < F > (f : F) -> Self :: Sequence where F : FnMut (usize) -> T , { S :: generate (f) } }
};
}
