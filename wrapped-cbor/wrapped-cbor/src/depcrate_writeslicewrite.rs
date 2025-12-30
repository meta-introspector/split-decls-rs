// Generated macro for SliceWrite (struct)
macro_rules! Depcrate_writeSliceWrite {
() => {
// Module: crate::write
// Provides: {"SliceWrite"}
// Dependencies: {}
# [doc = " Implements [`Write`](trait.Write.html) for mutable byte slices (`&mut [u8]`)."] # [doc = ""] # [doc = " Returns an error if the value to serialize is too large to fit in the slice."] # [derive (Debug)] pub struct SliceWrite < 'a > { slice : & 'a mut [u8] , index : usize , }
};
}
