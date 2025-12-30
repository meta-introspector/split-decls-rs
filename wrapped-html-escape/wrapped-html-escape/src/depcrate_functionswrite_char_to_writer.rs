// Generated macro for write_char_to_writer (function)
macro_rules! Depcrate_functionswrite_char_to_writer {
() => {
// Module: crate::functions
// Provides: {"write_char_to_writer"}
// Dependencies: {}
# [cfg (feature = "std")] # [inline] pub (crate) fn write_char_to_writer < W : Write > (c : char , output : & mut W) -> Result < () , io :: Error > { let mut buffer = [0u8 ; 4] ; let length = c . encode_utf8 (& mut buffer) . len () ; output . write_all (& buffer [.. length]) }
};
}
