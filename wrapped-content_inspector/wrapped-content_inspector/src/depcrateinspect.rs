// Generated macro for inspect (function)
macro_rules! Depcrateinspect {
() => {
// Module: crate
// Provides: {"inspect"}
// Dependencies: {}
# [doc = " Try to determine the type of content in the given buffer. See the crate documentation for a"] # [doc = " usage example and for more details on how this analysis is performed."] # [doc = ""] # [doc = " If the buffer is empty, the content type will be reported as `UTF_8`."] pub fn inspect (buffer : & [u8]) -> ContentType { use ContentType :: * ; for & (bom , content_type) in BYTE_ORDER_MARKS { if buffer . starts_with (bom) { return content_type ; } } let scan_size = min (buffer . len () , MAX_SCAN_SIZE) ; let has_zero_bytes = memchr (0x00 , & buffer [.. scan_size]) . is_some () ; if has_zero_bytes { return BINARY ; } if MAGIC_NUMBERS . iter () . any (| magic | buffer . starts_with (magic)) { return BINARY ; } UTF_8 }
};
}
