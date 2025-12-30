// Generated macro for impl_192 (impl)
macro_rules! Depcrate_string_recordimpl_192 {
() => {
// Module: crate::string_record
// Provides: {"impl_192"}
// Dependencies: {}
impl < 'r > Iterator for StringRecordIter < 'r > { type Item = & 'r str ; # [inline] fn next (& mut self) -> Option < & 'r str > { self . 0 . next () . map (| bytes | { debug_assert ! (str :: from_utf8 (bytes) . is_ok ()) ; unsafe { str :: from_utf8_unchecked (bytes) } }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> usize { self . 0 . len () } }
};
}
