// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a VecList < T > { type IntoIter = Iter < 'a , T > ; type Item = & 'a T ; fn into_iter (self) -> Self :: IntoIter { Iter { entries : & self . entries , head : self . head , remaining : self . length , tail : self . tail , } } }
};
}
