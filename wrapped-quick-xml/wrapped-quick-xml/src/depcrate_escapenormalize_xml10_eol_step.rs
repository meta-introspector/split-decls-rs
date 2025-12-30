// Generated macro for normalize_xml10_eol_step (function)
macro_rules! Depcrate_escapenormalize_xml10_eol_step {
() => {
// Module: crate::escape
// Provides: {"normalize_xml10_eol_step"}
// Dependencies: {}
# [doc = " The text in HTML normally is not normalized in any way; normalization is"] # [doc = " performed only in limited contexts and [only for] `\\r\\n` and `\\r`."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `normalized`: the string with the result of normalization"] # [doc = " - `input`: UTF-8 bytes of the string to be normalized"] # [doc = " - `index`: a byte index into `input` of character which is processed right now."] # [doc = "   It always points to the first byte of character in UTF-8 encoding"] # [doc = " - `ch`: a character that should be put to the string instead of newline sequence"] # [doc = ""] # [doc = " [only for]: https://html.spec.whatwg.org/#normalize-newlines"] fn normalize_xml10_eol_step (normalized : & mut String , input : & [u8] , index : usize , ch : char ,) -> usize { match input [index] { b'\r' => { normalized . push (ch) ; if index + 1 < input . len () && input [index + 1] == b'\n' { return index + 2 ; } index + 1 } b'\n' => { normalized . push (ch) ; index + 1 } x => unreachable ! ("at {}: expected ''\\n' or '\\r', found '{}' / {} / `0x{:X}`" , index , x as char , x , x) , } }
};
}
