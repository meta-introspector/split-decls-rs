// Generated macro for impl_230 (impl)
macro_rules! Depcrate_envimpl_230 {
() => {
// Module: crate::env
// Provides: {"impl_230"}
// Dependencies: {}
# [cfg (feature = "_cargo_insta_internal")] impl std :: str :: FromStr for UnreferencedSnapshots { type Err = () ; fn from_str (value : & str) -> Result < UnreferencedSnapshots , () > { match value { "auto" => Ok (UnreferencedSnapshots :: Auto) , "reject" | "error" => Ok (UnreferencedSnapshots :: Reject) , "delete" => Ok (UnreferencedSnapshots :: Delete) , "warn" => Ok (UnreferencedSnapshots :: Warn) , "ignore" => Ok (UnreferencedSnapshots :: Ignore) , _ => Err (()) , } } }
};
}
