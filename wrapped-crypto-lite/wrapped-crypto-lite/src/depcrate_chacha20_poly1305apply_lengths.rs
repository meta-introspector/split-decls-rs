// Generated macro for apply_lengths (function)
macro_rules! Depcrate_chacha20_poly1305apply_lengths {
() => {
// Module: crate::chacha20_poly1305
// Provides: {"apply_lengths"}
// Dependencies: {}
fn apply_lengths (mac : & mut Poly1305 , associated_data : & [u8] , data : & mut [u8]) { let associated_data_len : u64 = associated_data . len () . try_into () . unwrap () ; let data_len : u64 = data . len () . try_into () . unwrap () ; let mut block = [0u8 ; 16] ; block [.. 8] . copy_from_slice (& associated_data_len . to_le_bytes ()) ; block [8 ..] . copy_from_slice (& data_len . to_le_bytes ()) ; mac . update (& block , true , false) ; }
};
}
