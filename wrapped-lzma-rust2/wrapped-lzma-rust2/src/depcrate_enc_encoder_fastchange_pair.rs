// Generated macro for change_pair (function)
macro_rules! Depcrate_enc_encoder_fastchange_pair {
() => {
// Module: crate::enc::encoder_fast
// Provides: {"change_pair"}
// Dependencies: {}
fn change_pair (small_dist : u32 , big_dist : u32) -> bool { small_dist < (big_dist >> 7) }
};
}
