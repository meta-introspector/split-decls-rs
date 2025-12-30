// Generated macro for impl_74 (impl)
macro_rules! Depcrate_deimpl_74 {
() => {
// Module: crate::de
// Provides: {"impl_74"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshDeserialize for std :: net :: Ipv4Addr { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let mut buf = [0u8 ; 4] ; reader . read_exact (& mut buf) . map_err (unexpected_eof_to_unexpected_length_of_input) ? ; Ok (std :: net :: Ipv4Addr :: from (buf)) } }
};
}
