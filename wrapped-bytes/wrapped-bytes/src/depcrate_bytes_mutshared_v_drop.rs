// Generated macro for shared_v_drop (function)
macro_rules! Depcrate_bytes_mutshared_v_drop {
() => {
// Module: crate::bytes_mut
// Provides: {"shared_v_drop"}
// Dependencies: {}
unsafe fn shared_v_drop (data : & mut AtomicPtr < () > , _ptr : * const u8 , _len : usize) { data . with_mut (| shared | { release_shared (* shared as * mut Shared) ; }) ; }
};
}
