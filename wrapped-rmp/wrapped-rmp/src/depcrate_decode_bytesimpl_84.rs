// Generated macro for impl_84 (impl)
macro_rules! Depcrate_decode_bytesimpl_84 {
() => {
// Module: crate::decode::bytes
// Provides: {"impl_84"}
// Dependencies: {}
impl RmpRead for Bytes < '_ > { type Error = BytesReadError ; # [inline] fn read_u8 (& mut self) -> Result < u8 , Self :: Error > { if let Some ((& first , newly_remaining)) = self . bytes . split_first () { self . bytes = newly_remaining ; self . current_position += 1 ; Ok (first) } else { Err (BytesReadError :: InsufficientBytes { expected : 1 , actual : 0 , position : self . current_position , }) } } # [inline] fn read_exact_buf (& mut self , buf : & mut [u8]) -> Result < () , Self :: Error > { let to_read = buf . len () ; if to_read <= self . bytes . len () { let (src , newly_remaining) = self . bytes . split_at (to_read) ; self . bytes = newly_remaining ; self . current_position += to_read as u64 ; buf . copy_from_slice (src) ; Ok (()) } else { Err (BytesReadError :: InsufficientBytes { expected : to_read , actual : self . bytes . len () , position : self . current_position , }) } } }
};
}
