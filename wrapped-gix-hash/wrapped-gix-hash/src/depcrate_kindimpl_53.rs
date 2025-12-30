// Generated macro for impl_53 (impl)
macro_rules! Depcrate_kindimpl_53 {
() => {
// Module: crate::kind
// Provides: {"impl_53"}
// Dependencies: {}
impl FromStr for Kind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "sha1" | "SHA1" => Kind :: Sha1 , other => return Err (other . into ()) , }) } }
};
}
