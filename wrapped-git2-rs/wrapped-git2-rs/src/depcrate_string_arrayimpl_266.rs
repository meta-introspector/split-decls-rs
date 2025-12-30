// Generated macro for impl_266 (impl)
macro_rules! Depcrate_string_arrayimpl_266 {
() => {
// Module: crate::string_array
// Provides: {"impl_266"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a StringArray { type Item = Option < & 'a str > ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
