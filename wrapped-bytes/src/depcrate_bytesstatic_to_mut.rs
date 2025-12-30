// Generated macro for static_to_mut (function)
macro_rules! Depcrate_bytesstatic_to_mut {
() => {
// Module: crate::bytes
// Provides: {"static_to_mut"}
// Dependencies: {}
unsafe fn static_to_mut (_ : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { let slice = slice :: from_raw_parts (ptr , len) ; BytesMut :: from (slice) }
};
}
