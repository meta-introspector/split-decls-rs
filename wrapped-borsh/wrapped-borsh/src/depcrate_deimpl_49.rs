// Generated macro for impl_49 (impl)
macro_rules! Depcrate_deimpl_49 {
() => {
// Module: crate::de
// Provides: {"impl_49"}
// Dependencies: {}
impl BorshDeserialize for usize { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let u : u64 = BorshDeserialize :: deserialize_reader (reader) ? ; let u = usize :: try_from (u) . map_err (| _ | { Error :: new (ErrorKind :: InvalidData , ERROR_OVERFLOW_ON_MACHINE_WITH_32_BIT_USIZE ,) }) ? ; Ok (u) } }
};
}
