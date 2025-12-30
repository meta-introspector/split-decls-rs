// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Iterator { type Item = Result < JsValue , JsValue > ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Iter < 'a > { Iter { js : self , state : IterState :: new () , } } }
};
}
