// Generated macro for impl_25 (impl)
macro_rules! Depcrate_deimpl_25 {
() => {
// Module: crate::de
// Provides: {"impl_25"}
// Dependencies: {}
impl BorshDeserialize for u8 { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let mut buf = [0u8 ; 1] ; reader . read_exact (& mut buf) . map_err (unexpected_eof_to_unexpected_length_of_input) ? ; Ok (buf [0]) } # [inline] # [doc (hidden)] fn vec_from_reader < R : Read > (len : u32 , reader : & mut R) -> Result < Option < Vec < Self > > > { let len : usize = len . try_into () . map_err (| _ | ErrorKind :: InvalidData) ? ; let mut vec = vec ! [0u8 ; len . min (1024 * 1024)] ; let mut pos = 0 ; while pos < len { if pos == vec . len () { vec . resize (vec . len () . saturating_mul (2) . min (len) , 0) } match reader . read (& mut vec . as_mut_slice () [pos ..]) ? { 0 => { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_UNEXPECTED_LENGTH_OF_INPUT ,)) } read => { pos += read ; } } } Ok (Some (vec)) } # [inline] # [doc (hidden)] fn array_from_reader < R : Read , const N : usize > (reader : & mut R) -> Result < Option < [Self ; N] > > { let mut arr = [0u8 ; N] ; reader . read_exact (& mut arr) . map_err (unexpected_eof_to_unexpected_length_of_input) ? ; Ok (Some (arr)) } }
};
}
