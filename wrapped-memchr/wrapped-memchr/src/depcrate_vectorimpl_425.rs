// Generated macro for impl_425 (impl)
macro_rules! Depcrate_vectorimpl_425 {
() => {
// Module: crate::vector
// Provides: {"impl_425"}
// Dependencies: {}
impl SensibleMoveMask { # [doc = " Get the mask in a form suitable for computing offsets."] # [doc = ""] # [doc = " Basically, this normalizes to little endian. On big endian, this swaps"] # [doc = " the bytes."] # [inline (always)] fn get_for_offset (self) -> u32 { # [cfg (target_endian = "big")] { self . 0 . swap_bytes () } # [cfg (target_endian = "little")] { self . 0 } } }
};
}
