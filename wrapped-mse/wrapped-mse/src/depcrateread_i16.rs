// Generated macro for read_i16 (function)
macro_rules! Depcrateread_i16 {
() => {
// Module: crate
// Provides: {"read_i16"}
// Dependencies: {}
fn read_i16 (buffer : & [u8] , index : usize) -> i16 { const SIZE : usize = size_of :: < i16 > () ; let mut bytes : [u8 ; SIZE] = [0u8 ; SIZE] ; bytes . copy_from_slice (& buffer [(index * SIZE) .. (index * SIZE + SIZE)]) ; i16 :: from_ne_bytes (bytes) }
};
}
