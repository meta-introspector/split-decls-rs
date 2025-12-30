// Generated macro for impl_176 (impl)
macro_rules! Depcrate_sequenceimpl_176 {
() => {
// Module: crate::sequence
// Provides: {"impl_176"}
// Dependencies: {}
unsafe impl < 'a , T : 'a , S : GenericSequence < T > > GenericSequence < T > for & 'a mut S where & 'a mut S : IntoIterator , { type Length = S :: Length ; type Sequence = S :: Sequence ; # [inline (always)] fn generate < F > (f : F) -> Self :: Sequence where F : FnMut (usize) -> T , { S :: generate (f) } }
};
}
