// Generated macro for with_vec (function)
macro_rules! Depcrate_datawith_vec {
() => {
// Module: crate::data
// Provides: {"with_vec"}
// Dependencies: {}
# [cfg (feature = "block2")] # [cfg (feature = "alloc")] unsafe fn with_vec < T : objc2 :: Message > (obj : objc2 :: rc :: Allocated < T > , bytes : Vec < u8 >) -> Retained < T > { use core :: mem :: ManuallyDrop ; use block2 :: { DynBlock , RcBlock } ; let capacity = bytes . capacity () ; let dealloc = RcBlock :: new (move | bytes : * mut c_void , len : usize | { let _ = unsafe { < Vec < u8 > > :: from_raw_parts (bytes . cast () , len , capacity) } ; }) ; let dealloc : & DynBlock < dyn Fn (* mut c_void , usize) + 'static > = & dealloc ; let mut bytes = ManuallyDrop :: new (bytes) ; let len = bytes . len () ; let bytes_ptr : * mut c_void = bytes . as_mut_ptr () . cast () ; unsafe { objc2 :: msg_send ! [obj , initWithBytesNoCopy : bytes_ptr , length : len , deallocator : dealloc ,] } }
};
}
