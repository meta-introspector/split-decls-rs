// Generated macro for BlockBuffer (struct)
macro_rules! DepcrateBlockBuffer {
() => {
// Module: crate
// Provides: {"BlockBuffer"}
// Dependencies: {}
# [doc = " Buffer for block processing of data."] pub struct BlockBuffer < BS : ArraySize , K : BufferKind > { buffer : MaybeUninit < Array < u8 , BS > > , pos : K :: Pos , }
};
}
