// Generated macro for char2byte (function)
macro_rules! Depcrate_rustc_literal_escaperchar2byte {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"char2byte"}
// Dependencies: {}
# [doc = " Turn an ascii char into a byte"] # [inline] fn char2byte (c : char) -> Result < u8 , EscapeError > { if c . is_ascii () { Ok (c as u8) } else { Err (EscapeError :: NonAsciiCharInByte) } }
};
}
