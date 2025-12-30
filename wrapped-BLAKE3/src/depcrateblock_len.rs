// Generated macro for BLOCK_LEN (const)
macro_rules! DepcrateBLOCK_LEN {
() => {
// Module: crate
// Provides: {"BLOCK_LEN"}
// Dependencies: {}
# [doc = " The number of bytes in a block, 64."] # [doc = ""] # [doc = " You don't usually need to think about this number. One case where it matters is calling"] # [doc = " [`OutputReader::fill`] in a loop, where using a `buf` argument that's a multiple of `BLOCK_LEN`"] # [doc = " avoids repeating work."] pub const BLOCK_LEN : usize = 64 ;
};
}
