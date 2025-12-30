// Generated macro for simple_escape (function)
macro_rules! Depcratesimple_escape {
() => {
// Module: crate
// Provides: {"simple_escape"}
// Dependencies: {}
# [doc = " Interpret a non-nul ASCII escape"] # [doc = ""] # [doc = " Parses the character of an ASCII escape (except nul) without the leading backslash."] # [inline] fn simple_escape (c : char) -> Result < NonZero < u8 > , char > { Ok (NonZero :: new (match c { '"' => b'"' , 'n' => b'\n' , 'r' => b'\r' , 't' => b'\t' , '\\' => b'\\' , '\'' => b'\'' , _ => Err (c) ? , }) . unwrap ()) }
};
}
