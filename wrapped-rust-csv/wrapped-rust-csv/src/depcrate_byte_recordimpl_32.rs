// Generated macro for impl_32 (impl)
macro_rules! Depcrate_byte_recordimpl_32 {
() => {
// Module: crate::byte_record
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'r > Iterator for ByteRecordIter < 'r > { type Item = & 'r [u8] ; # [inline] fn next (& mut self) -> Option < & 'r [u8] > { if self . i_forward == self . i_reverse { None } else { let start = self . last_end ; let end = self . r . 0 . bounds . ends () [self . i_forward] ; self . i_forward += 1 ; self . last_end = end ; Some (& self . r . 0 . fields [start .. end]) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let x = self . i_reverse - self . i_forward ; (x , Some (x)) } # [inline] fn count (self) -> usize { self . len () } }
};
}
