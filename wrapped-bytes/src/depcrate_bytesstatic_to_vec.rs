// Generated macro for static_to_vec (function)
macro_rules! Depcrate_bytesstatic_to_vec {
() => {
// Module: crate::bytes
// Provides: {"static_to_vec"}
// Dependencies: {}
unsafe fn static_to_vec (_ : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Vec < u8 > { let slice = slice :: from_raw_parts (ptr , len) ; slice . to_vec () }
};
}
