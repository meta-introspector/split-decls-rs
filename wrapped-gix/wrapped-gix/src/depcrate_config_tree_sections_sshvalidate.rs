// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_sshvalidate {
() => {
// Module: crate::config::tree::sections::ssh
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct Variant ; impl keys :: Validate for Variant { fn validate (& self , _value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { # [cfg (feature = "blocking-network-client")] super :: Ssh :: VARIANT . try_into_variant (_value . into ()) ? ; Ok (()) } } }
};
}
