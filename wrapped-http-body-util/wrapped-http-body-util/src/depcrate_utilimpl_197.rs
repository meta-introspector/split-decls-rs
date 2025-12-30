// Generated macro for impl_197 (impl)
macro_rules! Depcrate_utilimpl_197 {
() => {
// Module: crate::util
// Provides: {"impl_197"}
// Dependencies: {}
impl < T : Buf > Buf for BufList < T > { # [inline] fn remaining (& self) -> usize { self . bufs . iter () . map (| buf | buf . remaining ()) . sum () } # [inline] fn has_remaining (& self) -> bool { self . bufs . iter () . any (| buf | buf . has_remaining ()) } # [inline] fn chunk (& self) -> & [u8] { self . bufs . front () . map (Buf :: chunk) . unwrap_or_default () } # [inline] fn advance (& mut self , mut cnt : usize) { while cnt > 0 { { let front = & mut self . bufs [0] ; let rem = front . remaining () ; if rem > cnt { front . advance (cnt) ; return ; } else { front . advance (rem) ; cnt -= rem ; } } self . bufs . pop_front () ; } } # [inline] fn chunks_vectored < 't > (& 't self , dst : & mut [IoSlice < 't >]) -> usize { if dst . is_empty () { return 0 ; } let mut vecs = 0 ; for buf in & self . bufs { vecs += buf . chunks_vectored (& mut dst [vecs ..]) ; if vecs == dst . len () { break ; } } vecs } # [inline] fn copy_to_bytes (& mut self , len : usize) -> Bytes { match self . bufs . front_mut () { Some (front) if front . remaining () == len => { let b = front . copy_to_bytes (len) ; self . bufs . pop_front () ; b } Some (front) if front . remaining () > len => front . copy_to_bytes (len) , _ => { let rem = self . remaining () ; assert ! (len <= rem , "`len` greater than remaining") ; let mut bm = BytesMut :: with_capacity (len) ; if rem == len { bm . put (self) ; } else { bm . put (self . take (len)) ; } bm . freeze () } } } }
};
}
