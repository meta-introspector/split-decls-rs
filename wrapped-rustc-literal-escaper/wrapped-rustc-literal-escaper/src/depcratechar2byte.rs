// Generated macro for char2byte (function)
macro_rules! Depcratechar2byte {
() => {
// Module: crate
// Provides: {"char2byte"}
// Dependencies: {}
# [doc = " Turn an ascii char into a byte"] # [inline] fn char2byte (c : char) -> Result < u8 , EscapeError > { if c . is_ascii () { Ok (c as u8) } else { Err (EscapeError :: NonAsciiCharInByte) } }
};
}
