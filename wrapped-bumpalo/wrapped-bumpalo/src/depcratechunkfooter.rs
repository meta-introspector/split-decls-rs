// Generated macro for ChunkFooter (struct)
macro_rules! DepcrateChunkFooter {
() => {
// Module: crate
// Provides: {"ChunkFooter"}
// Dependencies: {}
# [repr (C)] # [derive (Debug)] struct ChunkFooter { data : NonNull < u8 > , layout : Layout , prev : Cell < NonNull < ChunkFooter > > , ptr : Cell < NonNull < u8 > > , allocated_bytes : usize , }
};
}
