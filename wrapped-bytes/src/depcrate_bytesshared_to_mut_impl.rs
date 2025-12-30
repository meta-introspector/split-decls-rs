// Generated macro for shared_to_mut_impl (function)
macro_rules! Depcrate_bytesshared_to_mut_impl {
() => {
// Module: crate::bytes
// Provides: {"shared_to_mut_impl"}
// Dependencies: {}
unsafe fn shared_to_mut_impl (shared : * mut Shared , ptr : * const u8 , len : usize) -> BytesMut { if (* shared) . ref_cnt . load (Ordering :: Acquire) == 1 { let shared = * Box :: from_raw (shared) ; let shared = ManuallyDrop :: new (shared) ; let buf = shared . buf ; let cap = shared . cap ; let off = ptr . offset_from (buf) as usize ; let v = Vec :: from_raw_parts (buf , len + off , cap) ; let mut b = BytesMut :: from_vec (v) ; b . advance_unchecked (off) ; b } else { let v = slice :: from_raw_parts (ptr , len) . to_vec () ; release_shared (shared) ; BytesMut :: from_vec (v) } }
};
}
