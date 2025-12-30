// Generated macro for impl_71 (impl)
macro_rules! Depcrate_deimpl_71 {
() => {
// Module: crate::de
// Provides: {"impl_71"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshDeserialize for std :: net :: IpAddr { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let kind = u8 :: deserialize_reader (reader) ? ; match kind { 0u8 => { let ipv4_addr = std :: net :: Ipv4Addr :: deserialize_reader (reader) ? ; Ok (std :: net :: IpAddr :: V4 (ipv4_addr)) } 1u8 => { let ipv6_addr = std :: net :: Ipv6Addr :: deserialize_reader (reader) ? ; Ok (std :: net :: IpAddr :: V6 (ipv6_addr)) } value => Err (Error :: new (ErrorKind :: InvalidData , format ! ("Invalid IpAddr variant: {}" , value) ,)) , } } }
};
}
