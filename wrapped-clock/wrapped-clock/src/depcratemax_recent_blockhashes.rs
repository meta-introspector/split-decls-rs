// Generated macro for MAX_RECENT_BLOCKHASHES (const)
macro_rules! DepcrateMAX_RECENT_BLOCKHASHES {
() => {
// Module: crate
// Provides: {"MAX_RECENT_BLOCKHASHES"}
// Dependencies: {}
pub const MAX_RECENT_BLOCKHASHES : usize = MAX_HASH_AGE_IN_SECONDS * DEFAULT_TICKS_PER_SECOND as usize / DEFAULT_TICKS_PER_SLOT as usize ;
};
}
