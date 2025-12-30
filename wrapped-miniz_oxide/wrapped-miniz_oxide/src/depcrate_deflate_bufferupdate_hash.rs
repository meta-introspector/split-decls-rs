// Generated macro for update_hash (function)
macro_rules! Depcrate_deflate_bufferupdate_hash {
() => {
// Module: crate::deflate::buffer
// Provides: {"update_hash"}
// Dependencies: {}
# [inline] pub const fn update_hash (current_hash : u16 , byte : u8) -> u16 { ((current_hash << LZ_HASH_SHIFT) ^ byte as u16) & (LZ_HASH_SIZE as u16 - 1) }
};
}
