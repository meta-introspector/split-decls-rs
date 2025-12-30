// Generated macro for impl_229 (impl)
macro_rules! Depcrate_envimpl_229 {
() => {
// Module: crate::env
// Provides: {"impl_229"}
// Dependencies: {}
# [cfg (feature = "_cargo_insta_internal")] impl std :: str :: FromStr for TestRunner { type Err = () ; fn from_str (value : & str) -> Result < TestRunner , () > { match value { "auto" => Ok (TestRunner :: Auto) , "cargo-test" => Ok (TestRunner :: CargoTest) , "nextest" => Ok (TestRunner :: Nextest) , _ => Err (()) , } } }
};
}
