// Generated macro for impl_for_float (macro)
macro_rules! Depcrate_deimpl_for_float {
() => {
// Module: crate::de
// Provides: {"impl_for_float"}
// Dependencies: {}
macro_rules ! impl_for_float { ($ type : ident , $ int_type : ident) => { impl BorshDeserialize for $ type { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let mut buf = [0u8 ; size_of ::<$ type > ()] ; reader . read_exact (& mut buf) . map_err (unexpected_eof_to_unexpected_length_of_input) ?; let res = $ type :: from_bits ($ int_type :: from_le_bytes (buf . try_into () . unwrap ())) ; if res . is_nan () { return Err (Error :: new (ErrorKind :: InvalidData , "For portability reasons we do not allow to deserialize NaNs." ,)) ; } Ok (res) } } } ; }
};
}
