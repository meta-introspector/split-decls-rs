// Generated macro for promotable_to_mut (function)
macro_rules! Depcrate_bytespromotable_to_mut {
() => {
// Module: crate::bytes
// Provides: {"promotable_to_mut"}
// Dependencies: {}
unsafe fn promotable_to_mut (data : & AtomicPtr < () > , ptr : * const u8 , len : usize , f : fn (* mut ()) -> * mut u8 ,) -> BytesMut { let shared = data . load (Ordering :: Acquire) ; let kind = shared as usize & KIND_MASK ; if kind == KIND_ARC { shared_to_mut_impl (shared . cast () , ptr , len) } else { debug_assert_eq ! (kind , KIND_VEC) ; let buf = f (shared) ; let off = ptr . offset_from (buf) as usize ; let cap = off + len ; let v = Vec :: from_raw_parts (buf , cap , cap) ; let mut b = BytesMut :: from_vec (v) ; b . advance_unchecked (off) ; b } }
};
}
