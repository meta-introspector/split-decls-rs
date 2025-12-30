// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl core :: iter :: IntoIterator for Array { type Item = JsValue ; type IntoIter = ArrayIntoIter ; fn into_iter (self) -> Self :: IntoIter { ArrayIntoIter { range : 0 .. self . length () , array : self , } } }
};
}
