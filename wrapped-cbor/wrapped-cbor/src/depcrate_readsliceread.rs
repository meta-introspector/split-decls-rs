// Generated macro for SliceRead (struct)
macro_rules! Depcrate_readSliceRead {
() => {
// Module: crate::read
// Provides: {"SliceRead"}
// Dependencies: {}
# [doc = " A CBOR input source that reads from a slice of bytes."] # [cfg (any (feature = "std" , feature = "alloc"))] # [derive (Debug)] pub struct SliceRead < 'a > { slice : & 'a [u8] , scratch : Vec < u8 > , index : usize , }
};
}
