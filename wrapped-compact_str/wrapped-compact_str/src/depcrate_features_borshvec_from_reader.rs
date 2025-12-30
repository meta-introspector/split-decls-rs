// Generated macro for vec_from_reader (function)
macro_rules! Depcrate_features_borshvec_from_reader {
() => {
// Module: crate::features::borsh
// Provides: {"vec_from_reader"}
// Dependencies: {}
fn vec_from_reader < R : Read > (len : usize , reader : & mut R) -> Result < Vec < u8 > > { let mut vec = alloc :: vec ! [0u8 ; len . min (1024 * 1024)] ; let mut pos = 0 ; while pos < len { if pos == vec . len () { vec . resize (vec . len () . saturating_mul (2) . min (len) , 0) } match reader . read (& mut vec . as_mut_slice () [pos ..]) ? { 0 => { return Err (Error :: new (ErrorKind :: InvalidData , "Unexpected length of input" ,)) } read => { pos += read ; } } } Ok (vec) }
};
}
