// Generated macro for impl_48 (impl)
macro_rules! Depcrate_deimpl_48 {
() => {
// Module: crate::de
// Provides: {"impl_48"}
// Dependencies: {}
impl BorshDeserialize for isize { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let i : i64 = BorshDeserialize :: deserialize_reader (reader) ? ; let i = isize :: try_from (i) . map_err (| _ | { Error :: new (ErrorKind :: InvalidData , ERROR_OVERFLOW_ON_MACHINE_WITH_32_BIT_ISIZE ,) }) ? ; Ok (i) } }
};
}
