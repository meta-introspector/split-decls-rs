// Generated macro for impl_56 (impl)
macro_rules! Depcrate_deimpl_56 {
() => {
// Module: crate::de
// Provides: {"impl_56"}
// Dependencies: {}
impl BorshDeserialize for String { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { String :: from_utf8 (Vec :: < u8 > :: deserialize_reader (reader) ?) . map_err (| err | { let msg = err . to_string () ; Error :: new (ErrorKind :: InvalidData , msg) }) } }
};
}
