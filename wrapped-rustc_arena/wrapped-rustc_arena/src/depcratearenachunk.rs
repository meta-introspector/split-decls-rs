// Generated macro for ArenaChunk (struct)
macro_rules! DepcrateArenaChunk {
() => {
// Module: crate
// Provides: {"ArenaChunk"}
// Dependencies: {}
struct ArenaChunk < T = u8 > { # [doc = " The raw storage for the arena chunk."] storage : NonNull < [MaybeUninit < T >] > , # [doc = " The number of valid entries in the chunk."] entries : usize , }
};
}
