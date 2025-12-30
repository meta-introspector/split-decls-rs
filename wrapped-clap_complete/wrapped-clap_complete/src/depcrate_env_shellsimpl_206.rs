// Generated macro for impl_206 (impl)
macro_rules! Depcrate_env_shellsimpl_206 {
() => {
// Module: crate::env::shells
// Provides: {"impl_206"}
// Dependencies: {}
impl FromStr for CompType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "9" => Ok (Self :: Normal) , "63" => Ok (Self :: Successive) , "33" => Ok (Self :: Alternatives) , "64" => Ok (Self :: Unmodified) , "37" => Ok (Self :: Menu) , _ => Err (format ! ("unsupported COMP_TYPE `{s}`")) , } } }
};
}
