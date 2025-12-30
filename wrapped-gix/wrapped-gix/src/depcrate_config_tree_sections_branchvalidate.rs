// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_branchvalidate {
() => {
// Module: crate::config::tree::sections::branch
// Provides: {"validate"}
// Dependencies: {}
# [doc = ""] pub mod validate { use crate :: { bstr :: BStr , config :: tree :: { branch :: Merge , keys } , } ; pub struct FullNameRef ; impl keys :: Validate for FullNameRef { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Merge :: try_into_fullrefname (value . into ()) ? ; Ok (()) } } }
};
}
