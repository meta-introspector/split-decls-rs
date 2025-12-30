// Generated macro for DroplessArena (struct)
macro_rules! DepcrateDroplessArena {
() => {
// Module: crate
// Provides: {"DroplessArena"}
// Dependencies: {}
# [doc = " An arena that can hold objects of multiple different types that impl `Copy`"] # [doc = " and/or satisfy `!mem::needs_drop`."] pub struct DroplessArena { # [doc = " A pointer to the start of the free space."] start : Cell < * mut u8 > , # [doc = " A pointer to the end of free space."] # [doc = ""] # [doc = " The allocation proceeds downwards from the end of the chunk towards the"] # [doc = " start. (This is slightly simpler and faster than allocating upwards,"] # [doc = " see <https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html>.)"] # [doc = " When this pointer crosses the start pointer, a new chunk is allocated."] # [doc = ""] # [doc = " This is kept aligned to DROPLESS_ALIGNMENT."] end : Cell < * mut u8 > , # [doc = " A vector of arena chunks."] chunks : RefCell < Vec < ArenaChunk > > , }
};
}
