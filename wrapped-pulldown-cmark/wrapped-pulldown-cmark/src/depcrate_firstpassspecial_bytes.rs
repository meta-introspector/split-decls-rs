// Generated macro for special_bytes (function)
macro_rules! Depcrate_firstpassspecial_bytes {
() => {
// Module: crate::firstpass
// Provides: {"special_bytes"}
// Dependencies: {}
fn special_bytes (options : & Options) -> [bool ; 256] { let mut bytes = [false ; 256] ; let standard_bytes = [b'\n' , b'\r' , b'*' , b'_' , b'&' , b'\\' , b'[' , b']' , b'<' , b'!' , b'`' ,] ; for & byte in & standard_bytes { bytes [byte as usize] = true ; } if options . contains (Options :: ENABLE_TABLES) { bytes [b'|' as usize] = true ; } if options . contains (Options :: ENABLE_STRIKETHROUGH) || options . contains (Options :: ENABLE_SUBSCRIPT) { bytes [b'~' as usize] = true ; } if options . contains (Options :: ENABLE_SUPERSCRIPT) { bytes [b'^' as usize] = true ; } if options . contains (Options :: ENABLE_MATH) { bytes [b'$' as usize] = true ; bytes [b'{' as usize] = true ; bytes [b'}' as usize] = true ; } if options . contains (Options :: ENABLE_SMART_PUNCTUATION) { for & byte in & [b'.' , b'-' , b'"' , b'\''] { bytes [byte as usize] = true ; } } bytes }
};
}
