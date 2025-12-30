// Generated macro for static_clone (function)
macro_rules! Depcrate_bytesstatic_clone {
() => {
// Module: crate::bytes
// Provides: {"static_clone"}
// Dependencies: {}
unsafe fn static_clone (_ : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Bytes { let slice = slice :: from_raw_parts (ptr , len) ; Bytes :: from_static (slice) }
};
}
