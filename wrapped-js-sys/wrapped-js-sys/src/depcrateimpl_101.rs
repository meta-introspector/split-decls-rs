// Generated macro for impl_101 (impl)
macro_rules! Depcrateimpl_101 {
() => {
// Module: crate
// Provides: {"impl_101"}
// Dependencies: {}
impl IntoIterator for Iterator { type Item = Result < JsValue , JsValue > ; type IntoIter = IntoIter ; fn into_iter (self) -> IntoIter { IntoIter { js : self , state : IterState :: new () , } } }
};
}
