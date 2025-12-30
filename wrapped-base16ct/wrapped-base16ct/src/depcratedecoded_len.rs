// Generated macro for decoded_len (function)
macro_rules! Depcratedecoded_len {
() => {
// Module: crate
// Provides: {"decoded_len"}
// Dependencies: {}
# [doc = " Compute decoded length of the given hex-encoded input."] # [inline (always)] pub fn decoded_len (bytes : & [u8]) -> Result < usize > { if bytes . len () & 1 == 0 { Ok (bytes . len () / 2) } else { Err (Error :: InvalidLength) } }
};
}
