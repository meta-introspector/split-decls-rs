// Generated macro for ArenaLocal (struct)
macro_rules! Depcrate_arena_localArenaLocal {
() => {
// Module: crate::arena::local
// Provides: {"ArenaLocal"}
// Dependencies: {}
# [doc = " Thread-local arena allocator."] pub struct ArenaLocal { root : Cell < Option < NonNull < ChunkHeader > > > , min_chunk_size : Cell < usize > , }
};
}
