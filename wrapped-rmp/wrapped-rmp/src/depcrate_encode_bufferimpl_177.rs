// Generated macro for impl_177 (impl)
macro_rules! Depcrate_encode_bufferimpl_177 {
() => {
// Module: crate::encode::buffer
// Provides: {"impl_177"}
// Dependencies: {}
# [doc = " Fallback implementation for fixed-capacity buffers"] # [doc = ""] # [doc = " Only needed for no-std because we don't have"] # [doc = " the blanket impl for `std::io::Write`"] # [cfg (not (feature = "std"))] impl < 'a > RmpWrite for & 'a mut [u8] { type Error = FixedBufCapacityOverflow ; # [inline] fn write_bytes (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { let to_write = buf . len () ; let remaining = self . len () ; if to_write <= remaining { self [.. to_write] . copy_from_slice (buf) ; unsafe { * self = core :: slice :: from_raw_parts_mut (self . as_mut_ptr () . add (to_write) , remaining - to_write ,) } Ok (()) } else { Err (FixedBufCapacityOverflow { _priv : () }) } } }
};
}
