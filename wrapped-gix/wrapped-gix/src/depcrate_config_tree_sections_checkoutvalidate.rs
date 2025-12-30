// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_checkoutvalidate {
() => {
// Module: crate::config::tree::sections::checkout
// Provides: {"validate"}
// Dependencies: {}
# [doc = ""] pub mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct Workers ; impl keys :: Validate for Workers { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { super :: Checkout :: WORKERS . try_from_workers (gix_config :: Integer :: try_from (value) . and_then (| i | { i . to_decimal () . ok_or_else (| | gix_config :: value :: Error :: new ("Integer overflow" , value . to_owned ())) })) ? ; Ok (()) } } }
};
}
