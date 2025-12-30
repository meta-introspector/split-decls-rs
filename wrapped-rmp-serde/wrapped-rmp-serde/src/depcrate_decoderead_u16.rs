// Generated macro for read_u16 (function)
macro_rules! Depcrate_decoderead_u16 {
() => {
// Module: crate::decode
// Provides: {"read_u16"}
// Dependencies: {}
fn read_u16 < R : Read > (rd : & mut R) -> Result < u16 , Error > { rd . read_u16 :: < byteorder :: BigEndian > () . map_err (Error :: InvalidDataRead) }
};
}
