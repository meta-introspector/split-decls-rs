// Generated macro for decode_without_base64 (function)
macro_rules! Depcratedecode_without_base64 {
() => {
// Module: crate
// Provides: {"decode_without_base64"}
// Dependencies: {}
# [doc = " This is <https://url.spec.whatwg.org/#string-percent-decode> while also:"] # [doc = ""] # [doc = " * Ignoring ASCII tab or newlines"] # [doc = " * Stopping at the first '#' (which indicates the start of the fragment)"] # [doc = ""] # [doc = " Anything that would have been UTF-8 percent-encoded by the URL parser"] # [doc = " would be percent-decoded here."] # [doc = " We skip that round-trip and pass it through unchanged."] fn decode_without_base64 < F , E > (encoded_body_plus_fragment : & str , mut write_bytes : F ,) -> Result < Option < FragmentIdentifier < '_ > > , E > where F : FnMut (& [u8]) -> Result < () , E > , { let bytes = encoded_body_plus_fragment . as_bytes () ; let mut slice_start = 0 ; for (i , & byte) in bytes . iter () . enumerate () { if matches ! (byte , b'%' | b'#' | b'\t' | b'\n' | b'\r') { if i > slice_start { write_bytes (& bytes [slice_start .. i]) ? ; slice_start = i ; } match byte { b'%' => { let l = bytes . get (i + 2) . and_then (| & b | (b as char) . to_digit (16)) ; let h = bytes . get (i + 1) . and_then (| & b | (b as char) . to_digit (16)) ; if let (Some (h) , Some (l)) = (h , l) { let one_byte = h as u8 * 0x10 + l as u8 ; write_bytes (& [one_byte]) ? ; slice_start = i + 3 ; } else { } } b'#' => { let fragment_start = i + 1 ; let fragment = & encoded_body_plus_fragment [fragment_start ..] ; return Ok (Some (FragmentIdentifier (fragment))) ; } _ => slice_start = i + 1 , } } } write_bytes (& bytes [slice_start ..]) ? ; Ok (None) }
};
}
