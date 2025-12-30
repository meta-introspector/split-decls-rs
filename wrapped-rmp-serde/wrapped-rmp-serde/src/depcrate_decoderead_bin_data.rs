// Generated macro for read_bin_data (function)
macro_rules! Depcrate_decoderead_bin_data {
() => {
// Module: crate::decode
// Provides: {"read_bin_data"}
// Dependencies: {}
fn read_bin_data < 'a , 'de , R : ReadSlice < 'de > > (rd : & 'a mut R , len : u32) -> Result < Reference < 'de , 'a , [u8] > , Error > { rd . read_slice (len as usize) . map_err (Error :: InvalidDataRead) }
};
}
