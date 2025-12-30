// Generated macro for impl_193 (impl)
macro_rules! Depcrate_string_recordimpl_193 {
() => {
// Module: crate::string_record
// Provides: {"impl_193"}
// Dependencies: {}
impl < 'r > DoubleEndedIterator for StringRecordIter < 'r > { # [inline] fn next_back (& mut self) -> Option < & 'r str > { self . 0 . next_back () . map (| bytes | { debug_assert ! (str :: from_utf8 (bytes) . is_ok ()) ; unsafe { str :: from_utf8_unchecked (bytes) } }) } }
};
}
