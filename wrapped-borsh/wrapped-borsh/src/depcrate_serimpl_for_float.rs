// Generated macro for impl_for_float (macro)
macro_rules! Depcrate_serimpl_for_float {
() => {
// Module: crate::ser
// Provides: {"impl_for_float"}
// Dependencies: {}
macro_rules ! impl_for_float { ($ type : ident) => { impl BorshSerialize for $ type { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { if self . is_nan () { return Err (Error :: new (ErrorKind :: InvalidData , FLOAT_NAN_ERR)) ; } writer . write_all (& self . to_bits () . to_le_bytes ()) } } } ; }
};
}
