// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl < B : BitBlock > IntoIterator for BitVec < B > { type Item = bool ; type IntoIter = IntoIter < B > ; # [inline] fn into_iter (self) -> IntoIter < B > { let nbits = self . nbits ; IntoIter { bit_vec : self , range : 0 .. nbits , } } }
};
}
