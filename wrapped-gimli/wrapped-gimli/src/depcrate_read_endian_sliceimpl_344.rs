// Generated macro for impl_344 (impl)
macro_rules! Depcrate_read_endian_sliceimpl_344 {
() => {
// Module: crate::read::endian_slice
// Provides: {"impl_344"}
// Dependencies: {}
impl < 'input > core :: fmt :: Debug for DebugBytes < 'input > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> core :: result :: Result < () , fmt :: Error > { let mut list = fmt . debug_list () ; list . entries (self . 0 . iter () . take (8) . copied () . map (DebugByte)) ; if self . 0 . len () > 8 { list . entry (& DebugLen (self . 0 . len ())) ; } list . finish () } }
};
}
