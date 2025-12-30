// Generated macro for UnescapeBytes (struct)
macro_rules! Depcrate_escape_bytesUnescapeBytes {
() => {
// Module: crate::escape_bytes
// Provides: {"UnescapeBytes"}
// Dependencies: {}
# [doc = " An iterator of `u8` values that represent an unescaping of a sequence of"] # [doc = " codepoints."] # [doc = ""] # [doc = " The type parameter `I` refers to the iterator of codepoints that is"] # [doc = " unescaped."] # [doc = ""] # [doc = " Currently this iterator is not exposed in the crate API, and instead all"] # [doc = " we expose is a `ByteVec::unescape` method. Which of course requires an"] # [doc = " alloc. That's the most convenient form of this, but in theory, we could"] # [doc = " expose this for core-only use cases too. I'm just not quite sure what the"] # [doc = " API should be."] # [derive (Clone , Debug)] # [cfg (feature = "alloc")] pub (crate) struct UnescapeBytes < I > { it : I , state : UnescapeState , }
};
}
