// Generated macro for impl_72 (impl)
macro_rules! Depcrate_deimpl_72 {
() => {
// Module: crate::de
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshDeserialize for std :: net :: SocketAddrV4 { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let ip = std :: net :: Ipv4Addr :: deserialize_reader (reader) ? ; let port = u16 :: deserialize_reader (reader) ? ; Ok (std :: net :: SocketAddrV4 :: new (ip , port)) } }
};
}
