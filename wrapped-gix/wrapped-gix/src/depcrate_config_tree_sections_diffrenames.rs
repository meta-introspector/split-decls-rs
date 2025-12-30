// Generated macro for renames (module)
macro_rules! Depcrate_config_tree_sections_diffrenames {
() => {
// Module: crate::config::tree::sections::diff
// Provides: {"renames"}
// Dependencies: {}
mod renames { use crate :: { bstr :: ByteSlice , config :: { key :: GenericError , tree :: { keys , sections :: diff :: Renames , Section } , } , diff :: rename :: Tracking , } ; impl Renames { # [doc = " Create a new instance."] pub const fn new_renames (name : & 'static str , section : & 'static dyn Section) -> Self { keys :: Any :: new_with_validate (name , section , super :: validate :: Renames) } # [doc = " Try to convert the configuration into a valid rename tracking variant. Use `value` and if it's an error, interpret"] # [doc = " the boolean as string"] pub fn try_into_renames (& 'static self , value : Result < bool , gix_config :: value :: Error > ,) -> Result < Tracking , GenericError > { Ok (match value { Ok (true) => Tracking :: Renames , Ok (false) => Tracking :: Disabled , Err (err) => { let value = & err . input ; match value . as_bytes () { b"copy" | b"copies" => Tracking :: RenamesAndCopies , _ => return Err (GenericError :: from_value (self , value . clone ()) . with_source (err)) , } } }) } } }
};
}
