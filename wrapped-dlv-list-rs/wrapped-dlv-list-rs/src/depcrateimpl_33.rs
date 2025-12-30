// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > IntoIterator for VecList < T > { type IntoIter = IntoIter < T > ; type Item = T ; fn into_iter (self) -> Self :: IntoIter { IntoIter { head : self . head , remaining : self . length , tail : self . tail , list : self , } } }
};
}
