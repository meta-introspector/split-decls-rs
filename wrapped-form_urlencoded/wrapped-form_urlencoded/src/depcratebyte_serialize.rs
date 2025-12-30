// Generated macro for byte_serialize (function)
macro_rules! Depcratebyte_serialize {
() => {
// Module: crate
// Provides: {"byte_serialize"}
// Dependencies: {}
# [doc = " The [`application/x-www-form-urlencoded` byte serializer]("] # [doc = " https://url.spec.whatwg.org/#concept-urlencoded-byte-serializer)."] # [doc = ""] # [doc = " Return an iterator of `&str` slices."] pub fn byte_serialize (input : & [u8]) -> ByteSerialize < '_ > { ByteSerialize { bytes : input } }
};
}
