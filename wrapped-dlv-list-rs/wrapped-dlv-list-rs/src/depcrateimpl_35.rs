// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a mut VecList < T > { type IntoIter = IterMut < 'a , T > ; type Item = & 'a mut T ; fn into_iter (self) -> Self :: IntoIter { IterMut { entries : & mut self . entries , head : self . head , phantom : PhantomData , remaining : self . length , tail : self . tail , } } }
};
}
