// Generated macro for impl_for_nonzero_integer (macro)
macro_rules! Depcrate_deimpl_for_nonzero_integer {
() => {
// Module: crate::de
// Provides: {"impl_for_nonzero_integer"}
// Dependencies: {}
macro_rules ! impl_for_nonzero_integer { ($ type : ty) => { impl BorshDeserialize for $ type { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { <$ type >:: new (BorshDeserialize :: deserialize_reader (reader) ?) . ok_or_else (|| Error :: new (ErrorKind :: InvalidData , ERROR_INVALID_ZERO_VALUE)) } } } ; }
};
}
