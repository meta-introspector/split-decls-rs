// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl IntoIterator for Ini { type IntoIter = SectionIntoIter ; type Item = (SectionKey , Properties) ; fn into_iter (self) -> Self :: IntoIter { SectionIntoIter { inner : self . sections . into_iter () , } } }
};
}
