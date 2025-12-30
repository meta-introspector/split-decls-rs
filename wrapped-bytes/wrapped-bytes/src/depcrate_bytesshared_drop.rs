// Generated macro for shared_drop (function)
macro_rules! Depcrate_bytesshared_drop {
() => {
// Module: crate::bytes
// Provides: {"shared_drop"}
// Dependencies: {}
unsafe fn shared_drop (data : & mut AtomicPtr < () > , _ptr : * const u8 , _len : usize) { data . with_mut (| shared | { release_shared (shared . cast ()) ; }) ; }
};
}
