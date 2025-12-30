// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl IntoIterator for Properties { type IntoIter = PropertiesIntoIter ; type Item = (String , String) ; fn into_iter (self) -> Self :: IntoIter { PropertiesIntoIter { inner : self . data . into_iter () , } } }
};
}
