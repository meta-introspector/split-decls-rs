// Generated macro for impl_85 (impl)
macro_rules! Depcrate_decode_bytesimpl_85 {
() => {
// Module: crate::decode::bytes
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < 'a > RmpRead for & 'a [u8] { type Error = BytesReadError ; fn read_u8 (& mut self) -> Result < u8 , Self :: Error > { if let Some ((& first , newly_remaining)) = self . split_first () { * self = newly_remaining ; Ok (first) } else { Err (BytesReadError :: InsufficientBytes { expected : 1 , actual : 0 , position : 0 , }) } } fn read_exact_buf (& mut self , buf : & mut [u8]) -> Result < () , Self :: Error > { let to_read = buf . len () ; if to_read <= self . len () { let (src , newly_remaining) = self . split_at (to_read) ; * self = newly_remaining ; buf . copy_from_slice (src) ; Ok (()) } else { Err (BytesReadError :: InsufficientBytes { expected : to_read , actual : self . len () , position : 0 , }) } } }
};
}
