// Generated macro for algorithm (module)
macro_rules! Depcrate_config_tree_sections_fetchalgorithm {
() => {
// Module: crate::config::tree::sections::fetch
// Provides: {"algorithm"}
// Dependencies: {}
mod algorithm { # [cfg (feature = "credentials")] impl crate :: config :: tree :: sections :: fetch :: NegotiationAlgorithm { # [doc = " Derive the negotiation algorithm identified by `name`, case-sensitively."] pub fn try_into_negotiation_algorithm (& 'static self , name : std :: borrow :: Cow < '_ , crate :: bstr :: BStr > ,) -> Result < crate :: remote :: fetch :: negotiate :: Algorithm , crate :: config :: key :: GenericErrorWithValue > { use crate :: { bstr :: ByteSlice , remote :: fetch :: negotiate :: Algorithm } ; Ok (match name . as_ref () . as_bytes () { b"noop" => Algorithm :: Noop , b"consecutive" | b"default" => Algorithm :: Consecutive , b"skipping" => Algorithm :: Skipping , _ => { return Err (crate :: config :: key :: GenericErrorWithValue :: from_value (self , name . into_owned () ,)) } }) } } # [cfg (feature = "attributes")] impl crate :: config :: tree :: sections :: fetch :: RecurseSubmodules { # [doc = " Obtain the way submodules should be updated."] pub fn try_into_recurse_submodules (& 'static self , value : Result < bool , gix_config :: value :: Error > ,) -> Result < gix_submodule :: config :: FetchRecurse , crate :: config :: key :: GenericErrorWithValue > { gix_submodule :: config :: FetchRecurse :: new (value) . map_err (| err | crate :: config :: key :: GenericErrorWithValue :: from_value (self , err)) } } }
};
}
