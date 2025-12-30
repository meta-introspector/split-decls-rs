// Generated macro for impl_70 (impl)
macro_rules! Depcrate_deimpl_70 {
() => {
// Module: crate::de
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshDeserialize for std :: net :: SocketAddr { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let kind = u8 :: deserialize_reader (reader) ? ; match kind { 0 => std :: net :: SocketAddrV4 :: deserialize_reader (reader) . map (std :: net :: SocketAddr :: V4) , 1 => std :: net :: SocketAddrV6 :: deserialize_reader (reader) . map (std :: net :: SocketAddr :: V6) , value => Err (Error :: new (ErrorKind :: InvalidData , format ! ("Invalid SocketAddr variant: {}" , value) ,)) , } } }
};
}
