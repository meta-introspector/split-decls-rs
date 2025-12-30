// Generated macro for impl_for_integer (macro)
macro_rules! Depcrate_deimpl_for_integer {
() => {
// Module: crate::de
// Provides: {"impl_for_integer"}
// Dependencies: {}
macro_rules ! impl_for_integer { ($ type : ident) => { impl BorshDeserialize for $ type { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let mut buf = [0u8 ; size_of ::<$ type > ()] ; reader . read_exact (& mut buf) . map_err (unexpected_eof_to_unexpected_length_of_input) ?; let res = $ type :: from_le_bytes (buf . try_into () . unwrap ()) ; Ok (res) } } } ; }
};
}
