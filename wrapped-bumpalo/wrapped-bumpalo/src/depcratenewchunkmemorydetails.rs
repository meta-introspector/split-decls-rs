// Generated macro for NewChunkMemoryDetails (struct)
macro_rules! DepcrateNewChunkMemoryDetails {
() => {
// Module: crate
// Provides: {"NewChunkMemoryDetails"}
// Dependencies: {}
# [doc = " The memory size and alignment details for a potential new chunk"] # [doc = " allocation."] # [derive (Debug , Clone , Copy)] struct NewChunkMemoryDetails { new_size_without_footer : usize , align : usize , size : usize , }
};
}
