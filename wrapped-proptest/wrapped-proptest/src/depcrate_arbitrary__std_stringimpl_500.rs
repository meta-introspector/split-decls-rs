// Generated macro for impl_500 (impl)
macro_rules! Depcrate_arbitrary__std_stringimpl_500 {
() => {
// Module: crate::arbitrary::_std::string
// Provides: {"impl_500"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a ELBytes { type Item = u8 ; type IntoIter = iter :: Cloned < slice :: Iter < 'a , u8 > > ; fn into_iter (self) -> Self :: IntoIter { use self :: ELBytes :: * ; (match * self { B1 (ref a) => a . iter () , B2 (ref a) => a . iter () , B3 (ref a) => a . iter () , B4 (ref a) => a . iter () , }) . cloned () } }
};
}
