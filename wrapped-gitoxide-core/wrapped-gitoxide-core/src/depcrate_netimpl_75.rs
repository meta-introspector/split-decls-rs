// Generated macro for impl_75 (impl)
macro_rules! Depcrate_netimpl_75 {
() => {
// Module: crate::net
// Provides: {"impl_75"}
// Dependencies: {}
impl FromStr for Protocol { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "1" => Protocol :: V1 , "2" => Protocol :: V2 , _ => return Err (format ! ("Unsupported protocol version '{s}', choose '1' or '2'")) , }) } }
};
}
