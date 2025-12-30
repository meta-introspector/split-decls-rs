// Generated macro for impl_73 (impl)
macro_rules! Depcrate_buf_takeimpl_73 {
() => {
// Module: crate::buf::take
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : Buf > Buf for Take < T > { fn remaining (& self) -> usize { cmp :: min (self . inner . remaining () , self . limit) } fn chunk (& self) -> & [u8] { let bytes = self . inner . chunk () ; & bytes [.. cmp :: min (bytes . len () , self . limit)] } fn advance (& mut self , cnt : usize) { assert ! (cnt <= self . limit) ; self . inner . advance (cnt) ; self . limit -= cnt ; } fn copy_to_bytes (& mut self , len : usize) -> crate :: Bytes { assert ! (len <= self . remaining () , "`len` greater than remaining") ; let r = self . inner . copy_to_bytes (len) ; self . limit -= len ; r } # [cfg (feature = "std")] fn chunks_vectored < 'a > (& 'a self , dst : & mut [IoSlice < 'a >]) -> usize { if self . limit == 0 { return 0 ; } const LEN : usize = 16 ; let mut slices : [IoSlice < 'a > ; LEN] = [IoSlice :: new (& []) ; LEN] ; let cnt = self . inner . chunks_vectored (& mut slices [.. dst . len () . min (LEN)]) ; let mut limit = self . limit ; for (i , (dst , slice)) in dst [.. cnt] . iter_mut () . zip (slices . iter ()) . enumerate () { if let Some (buf) = slice . get (.. limit) { let buf = unsafe { std :: mem :: transmute :: < & [u8] , & 'a [u8] > (buf) } ; * dst = IoSlice :: new (buf) ; return i + 1 ; } else { let buf = unsafe { std :: mem :: transmute :: < & [u8] , & 'a [u8] > (slice) } ; * dst = IoSlice :: new (buf) ; limit -= slice . len () ; } } cnt } }
};
}
