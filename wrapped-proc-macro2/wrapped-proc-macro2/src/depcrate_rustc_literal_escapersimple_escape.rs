// Generated macro for simple_escape (function)
macro_rules! Depcrate_rustc_literal_escapersimple_escape {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"simple_escape"}
// Dependencies: {}
# [doc = " Interpret a non-nul ASCII escape"] # [doc = ""] # [doc = " Parses the character of an ASCII escape (except nul) without the leading backslash."] # [inline] fn simple_escape (c : char) -> Result < NonZeroU8 , char > { Ok (NonZeroU8 :: new (match c { '"' => b'"' , 'n' => b'\n' , 'r' => b'\r' , 't' => b'\t' , '\\' => b'\\' , '\'' => b'\'' , _ => Err (c) ? , }) . unwrap ()) }
};
}
