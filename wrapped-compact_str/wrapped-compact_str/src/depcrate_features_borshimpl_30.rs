// Generated macro for impl_30 (impl)
macro_rules! Depcrate_features_borshimpl_30 {
() => {
// Module: crate::features::borsh
// Provides: {"impl_30"}
// Dependencies: {}
impl BorshDeserialize for CompactString { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let len = u32 :: deserialize_reader (& mut * reader) ? as usize ; if len <= MAX_SIZE { let mut buf = [0u8 ; MAX_SIZE] ; reader . read_exact (& mut buf [.. len]) ? ; let s = str :: from_utf8 (& buf [.. len]) . map_err (| err | Error :: new (ErrorKind :: InvalidData , err)) ? ; Ok (CompactString :: from (s)) } else { let buf = vec_from_reader (len , reader) ? ; let s = String :: from_utf8 (buf) . map_err (| err | Error :: new (ErrorKind :: InvalidData , err)) ? ; Ok (CompactString :: from (s)) } } }
};
}
