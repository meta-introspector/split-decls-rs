// Generated macro for SliceReader (struct)
macro_rules! Depcrate_de_readSliceReader {
() => {
// Module: crate::de::read
// Provides: {"SliceReader"}
// Dependencies: {}
# [doc = " A reader type for `&[u8]` slices. Implements both [Reader] and [BorrowReader], and thus can be used for borrowed data."] pub struct SliceReader < 'storage > { pub (crate) slice : & 'storage [u8] , }
};
}
