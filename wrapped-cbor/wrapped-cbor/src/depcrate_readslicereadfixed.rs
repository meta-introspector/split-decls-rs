// Generated macro for SliceReadFixed (struct)
macro_rules! Depcrate_readSliceReadFixed {
() => {
// Module: crate::read
// Provides: {"SliceReadFixed"}
// Dependencies: {}
# [doc = " A CBOR input source that reads from a slice of bytes using a fixed size scratch buffer."] # [doc = ""] # [doc = " [`SliceRead`](struct.SliceRead.html) and [`MutSliceRead`](struct.MutSliceRead.html) are usually"] # [doc = " preferred over this, as they can handle indefinite length items."] # [derive (Debug)] pub struct SliceReadFixed < 'a , 'b > { slice : & 'a [u8] , scratch : & 'b mut [u8] , index : usize , scratch_index : usize , }
};
}
