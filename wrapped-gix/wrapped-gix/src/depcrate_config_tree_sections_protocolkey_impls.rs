// Generated macro for key_impls (module)
macro_rules! Depcrate_config_tree_sections_protocolkey_impls {
() => {
// Module: crate::config::tree::sections::protocol
// Provides: {"key_impls"}
// Dependencies: {}
mod key_impls { impl super :: Version { # [doc = " Convert `value` into the corresponding protocol version, possibly applying the correct default."] # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub fn try_into_protocol_version (& 'static self , value : Option < Result < i64 , gix_config :: value :: Error > > ,) -> Result < gix_protocol :: transport :: Protocol , crate :: config :: key :: GenericErrorWithValue > { let value = match value { None => return Ok (gix_protocol :: transport :: Protocol :: V2) , Some (v) => v , } ; Ok (match value { Ok (0) => gix_protocol :: transport :: Protocol :: V0 , Ok (1) => gix_protocol :: transport :: Protocol :: V1 , Ok (2) => gix_protocol :: transport :: Protocol :: V2 , Ok (other) => { return Err (crate :: config :: key :: GenericErrorWithValue :: from_value (self , other . to_string () . into () ,)) } Err (err) => { return Err (crate :: config :: key :: GenericErrorWithValue :: from_value (self , "unknown" . into ()) . with_source (err) ,) } }) } } }
};
}
