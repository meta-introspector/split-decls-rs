// Generated macro for impl_75 (impl)
macro_rules! Depcrate_deimpl_75 {
() => {
// Module: crate::de
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshDeserialize for std :: net :: Ipv6Addr { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let mut buf = [0u8 ; 16] ; reader . read_exact (& mut buf) . map_err (unexpected_eof_to_unexpected_length_of_input) ? ; Ok (std :: net :: Ipv6Addr :: from (buf)) } }
};
}
