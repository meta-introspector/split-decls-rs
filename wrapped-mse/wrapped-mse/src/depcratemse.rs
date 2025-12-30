// Generated macro for mse (function)
macro_rules! Depcratemse {
() => {
// Module: crate
// Provides: {"mse"}
// Dependencies: {}
fn mse (samples : usize , frame_buf : & [i16] , buf_ref : & [u8]) -> f64 { let mut mse = 0.0 ; let max_samples = std :: cmp :: min (buf_ref . len () / 2 , samples as usize) ; for i in 0 .. max_samples { let ref_res = read_i16 (buf_ref , i) ; let info_res = frame_buf [i as usize] ; let diff = (ref_res - info_res) . abs () ; mse += f64 :: from (diff . pow (2)) ; } mse / max_samples as f64 }
};
}
