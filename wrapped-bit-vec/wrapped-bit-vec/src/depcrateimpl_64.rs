// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , B : BitBlock > IntoIterator for & 'a BitVec < B > { type Item = bool ; type IntoIter = Iter < 'a , B > ; # [inline] fn into_iter (self) -> Iter < 'a , B > { self . iter () } }
};
}
