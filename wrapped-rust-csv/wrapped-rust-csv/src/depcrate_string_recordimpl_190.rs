// Generated macro for impl_190 (impl)
macro_rules! Depcrate_string_recordimpl_190 {
() => {
// Module: crate::string_record
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a StringRecord { type IntoIter = StringRecordIter < 'a > ; type Item = & 'a str ; # [inline] fn into_iter (self) -> StringRecordIter < 'a > { StringRecordIter (self . 0 . iter ()) } }
};
}
