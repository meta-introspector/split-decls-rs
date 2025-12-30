// Generated macro for Param (struct)
macro_rules! DepcrateParam {
() => {
// Module: crate
// Provides: {"Param"}
// Dependencies: {}
# [doc = " A parsed formatting parameter (contents of `{` `}` block)."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " ```notrust"] # [doc = " param := '{' [ argument ] [ '=' argtype ] [ ':' format_spec ] '}'"] # [doc = " argument := integer"] # [doc = ""] # [doc = " argtype := bitfield | '?' | format-array | '[?]' | byte-array | '[u8]' | 'istr' | 'str' |"] # [doc = "     'bool' | 'char' | 'u8' | 'u16' | 'u32' | 'u64' | 'u128' | 'usize' | 'i8' | 'i16' | 'i32' |"] # [doc = "     'i64' | 'i128 | 'isize' | 'f32' | 'f64'"] # [doc = " bitfield := integer '..' integer"] # [doc = " format-array := '[?;' spaces integer ']'"] # [doc = " byte-array := '[u8;' spaces integer ']'"] # [doc = " spaces := ' '*"] # [doc = ""] # [doc = " format_spec := [ zero_pad ] type"] # [doc = " zero_pad := '0' integer"] # [doc = " type := 'a' | 'b' | 'o' | 'x' | 'X' | '?' | 'us'"] # [doc = " ```"] # [derive (Debug , PartialEq)] struct Param { index : Option < usize > , ty : Type , hint : Option < DisplayHint > , }
};
}
