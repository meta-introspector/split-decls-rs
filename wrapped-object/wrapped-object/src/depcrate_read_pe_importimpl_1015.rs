// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_read_pe_importimpl_1015 {
() => {
// Module: crate::read::pe::import
// Provides: {"impl_1015"}
// Dependencies: {}
impl ImageThunkData for pe :: ImageThunkData32 { fn raw (self) -> u64 { self . 0 . get (LE) . into () } fn is_ordinal (self) -> bool { self . 0 . get (LE) & pe :: IMAGE_ORDINAL_FLAG32 != 0 } fn ordinal (self) -> u16 { self . 0 . get (LE) as u16 } fn address (self) -> u32 { self . 0 . get (LE) & 0x7fff_ffff } }
};
}
