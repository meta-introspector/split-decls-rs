// Generated macro for build_from_derive_input (function)
macro_rules! Depcrate_item_typebuild_from_derive_input {
() => {
// Module: crate::item_type
// Provides: {"build_from_derive_input"}
// Dependencies: {}
fn build_from_derive_input (item : DeriveInput) -> Result < TokenStream > { let mut kinds = HelperAttributeKinds :: new (true) ; match & item . data { Data :: Struct (data) => { build_by_item_struct_core (None , & to_item_struct (& item , data) , & mut kinds) } Data :: Enum (data) => build_by_item_enum_core (None , & to_item_enum (& item , data) , & mut kinds) , Data :: Union (_) => bail ! (Span :: call_site () , "does not support union types") , } }
};
}
