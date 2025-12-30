// Generated macro for decode_with_base64 (function)
macro_rules! Depcratedecode_with_base64 {
() => {
// Module: crate
// Provides: {"decode_with_base64"}
// Dependencies: {}
# [doc = " `decode_without_base64()` composed with"] # [doc = " <https://infra.spec.whatwg.org/#isomorphic-decode> composed with"] # [doc = " <https://infra.spec.whatwg.org/#forgiving-base64-decode>."] fn decode_with_base64 < F , E > (encoded_body_plus_fragment : & str , write_bytes : F ,) -> Result < Option < FragmentIdentifier < '_ > > , forgiving_base64 :: DecodeError < E > > where F : FnMut (& [u8]) -> Result < () , E > , { let mut decoder = forgiving_base64 :: Decoder :: new (write_bytes) ; let fragment = decode_without_base64 (encoded_body_plus_fragment , | bytes | decoder . feed (bytes)) ? ; decoder . finish () ? ; Ok (fragment) }
};
}
