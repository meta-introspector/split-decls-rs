// Generated macro for update_crc_with_padding (function)
macro_rules! Depcrate_xzupdate_crc_with_padding {
() => {
// Module: crate::xz
// Provides: {"update_crc_with_padding"}
// Dependencies: {}
fn update_crc_with_padding (crc : & mut crc :: Digest < '_ , u32 > , padding_needed : usize) { match padding_needed { 1 => crc . update (& [0]) , 2 => crc . update (& [0 , 0]) , 3 => crc . update (& [0 , 0 , 0]) , _ => { } } }
};
}
