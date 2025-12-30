// Generated macro for impl_151 (impl)
macro_rules! Depcrate_file_section_bodyimpl_151 {
() => {
// Module: crate::file::section::body
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'event > IntoIterator for Body < 'event > { type Item = (ValueName < 'event > , Cow < 'event , BStr >) ; type IntoIter = BodyIter < 'event > ; fn into_iter (self) -> Self :: IntoIter { BodyIter (self . 0 . into_iter ()) } }
};
}
