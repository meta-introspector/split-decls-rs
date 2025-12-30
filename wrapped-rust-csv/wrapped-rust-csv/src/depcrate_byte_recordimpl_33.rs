// Generated macro for impl_33 (impl)
macro_rules! Depcrate_byte_recordimpl_33 {
() => {
// Module: crate::byte_record
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'r > DoubleEndedIterator for ByteRecordIter < 'r > { # [inline] fn next_back (& mut self) -> Option < & 'r [u8] > { if self . i_forward == self . i_reverse { None } else { self . i_reverse -= 1 ; let start = self . i_reverse . checked_sub (1) . map (| i | self . r . 0 . bounds . ends () [i]) . unwrap_or (0) ; let end = self . last_start ; self . last_start = start ; Some (& self . r . 0 . fields [start .. end]) } } }
};
}
