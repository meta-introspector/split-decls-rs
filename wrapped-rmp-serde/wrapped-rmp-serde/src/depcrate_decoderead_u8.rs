// Generated macro for read_u8 (function)
macro_rules! Depcrate_decoderead_u8 {
() => {
// Module: crate::decode
// Provides: {"read_u8"}
// Dependencies: {}
fn read_u8 < R : Read > (rd : & mut R) -> Result < u8 , Error > { byteorder :: ReadBytesExt :: read_u8 (rd) . map_err (Error :: InvalidDataRead) }
};
}
