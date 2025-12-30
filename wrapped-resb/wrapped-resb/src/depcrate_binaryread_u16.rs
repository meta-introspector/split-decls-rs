// Generated macro for read_u16 (function)
macro_rules! Depcrate_binaryread_u16 {
() => {
// Module: crate::binary
// Provides: {"read_u16"}
// Dependencies: {}
# [doc = " Reads the first two bytes of the input and interprets them as a `u16` with"] # [doc = " native endianness."] # [doc = ""] # [doc = " Returns the `u16` and a slice containing all input after the interpreted"] # [doc = " bytes. Returns an error if the input is of insufficient length."] fn read_u16 (input : & [u8]) -> Result < (u16 , & [u8]) , BinaryDeserializerError > { # [expect (clippy :: unwrap_used)] let bytes = get_subslice (input , .. core :: mem :: size_of :: < u16 > ()) ? . try_into () . unwrap () ; let value = u16 :: from_le_bytes (bytes) ; let rest = get_subslice (input , core :: mem :: size_of :: < u16 > () ..) ? ; Ok ((value , rest)) }
};
}
