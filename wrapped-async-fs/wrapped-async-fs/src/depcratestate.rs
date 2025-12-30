// Generated macro for State (enum)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state of an asynchronous `ReadDir`."] # [doc = ""] # [doc = " The `ReadDir` can be either idle or busy performing an asynchronous operation."] # [allow (clippy :: large_enum_variant)] enum State { Idle (Option < std :: fs :: ReadDir >) , Busy (blocking :: Task < (std :: fs :: ReadDir , Option < io :: Result < std :: fs :: DirEntry > >) >) , }
};
}
