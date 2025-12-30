// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_protocolvalidate {
() => {
// Module: crate::config::tree::sections::protocol
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct Allow ; impl keys :: Validate for Allow { fn validate (& self , _value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] super :: Protocol :: ALLOW . try_into_allow (std :: borrow :: Cow :: Borrowed (_value) , None) ? ; Ok (()) } } pub struct Version ; impl keys :: Validate for Version { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { let value = gix_config :: Integer :: try_from (value) ? . to_decimal () . ok_or_else (| | format ! ("integer {value} cannot be represented as integer")) ? ; match value { 0 ..= 2 => Ok (()) , _ => Err (format ! ("protocol version {value} is unknown") . into ()) , } } } }
};
}
