// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Ini { type IntoIter = SectionIter < 'a > ; type Item = (Option < & 'a str > , & 'a Properties) ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
