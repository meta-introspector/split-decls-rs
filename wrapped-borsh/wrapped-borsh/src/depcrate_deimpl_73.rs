// Generated macro for impl_73 (impl)
macro_rules! Depcrate_deimpl_73 {
() => {
// Module: crate::de
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshDeserialize for std :: net :: SocketAddrV6 { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let ip = std :: net :: Ipv6Addr :: deserialize_reader (reader) ? ; let port = u16 :: deserialize_reader (reader) ? ; Ok (std :: net :: SocketAddrV6 :: new (ip , port , 0 , 0)) } }
};
}
