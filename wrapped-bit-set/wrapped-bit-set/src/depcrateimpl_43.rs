// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a , B : BitBlock > IntoIterator for & 'a BitSet < B > { type Item = usize ; type IntoIter = Iter < 'a , B > ; fn into_iter (self) -> Iter < 'a , B > { self . iter () } }
};
}
