// Generated macro for read_u32 (function)
macro_rules! Depcrate_decoderead_u32 {
() => {
// Module: crate::decode
// Provides: {"read_u32"}
// Dependencies: {}
fn read_u32 < R : Read > (rd : & mut R) -> Result < u32 , Error > { rd . read_u32 :: < byteorder :: BigEndian > () . map_err (Error :: InvalidDataRead) }
};
}
