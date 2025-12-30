// Generated macro for read_bin_data (function)
macro_rules! Depcrate_decode_valueread_bin_data {
() => {
// Module: crate::decode::value
// Provides: {"read_bin_data"}
// Dependencies: {}
fn read_bin_data < R : Read > (rd : & mut R , len : usize , depth : u16) -> Result < Vec < u8 > , Error > { let _depth = super :: decrement_depth (depth) ? ; let mut buf = Vec :: with_capacity (min (len , PREALLOC_MAX)) ; let bytes_read = rd . take (len as u64) . read_to_end (& mut buf) . map_err (Error :: InvalidDataRead) ? ; if bytes_read != len { return Err (Error :: InvalidDataRead (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , format ! ("Expected {len} bytes, read {bytes_read} bytes") ,))) ; } Ok (buf) }
};
}
