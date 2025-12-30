// Generated macro for decode_utf8_lossy (function)
macro_rules! Depcratedecode_utf8_lossy {
() => {
// Module: crate
// Provides: {"decode_utf8_lossy"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [allow (ambiguous_wide_pointer_comparisons)] fn decode_utf8_lossy (input : Cow < '_ , [u8] >) -> Cow < '_ , str > { match input { Cow :: Borrowed (bytes) => String :: from_utf8_lossy (bytes) , Cow :: Owned (bytes) => { match String :: from_utf8_lossy (& bytes) { Cow :: Borrowed (utf8) => { let raw_utf8 : * const [u8] = utf8 . as_bytes () ; debug_assert ! (core :: ptr :: eq (raw_utf8 , &* bytes)) ; Cow :: Owned (unsafe { String :: from_utf8_unchecked (bytes) }) } Cow :: Owned (s) => Cow :: Owned (s) , } } } }
};
}
