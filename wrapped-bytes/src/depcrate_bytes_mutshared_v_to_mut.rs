// Generated macro for shared_v_to_mut (function)
macro_rules! Depcrate_bytes_mutshared_v_to_mut {
() => {
// Module: crate::bytes_mut
// Provides: {"shared_v_to_mut"}
// Dependencies: {}
unsafe fn shared_v_to_mut (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { let shared : * mut Shared = data . load (Ordering :: Relaxed) . cast () ; if (* shared) . is_unique () { let shared = & mut * shared ; let v = & mut shared . vec ; let v_capacity = v . capacity () ; let v_ptr = v . as_mut_ptr () ; let offset = ptr . offset_from (v_ptr) as usize ; let cap = v_capacity - offset ; let ptr = vptr (ptr as * mut u8) ; BytesMut { ptr , len , cap , data : shared , } } else { let v = slice :: from_raw_parts (ptr , len) . to_vec () ; release_shared (shared) ; BytesMut :: from_vec (v) } }
};
}
