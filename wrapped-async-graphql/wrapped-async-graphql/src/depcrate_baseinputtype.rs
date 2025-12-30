// Generated macro for InputType (trait)
macro_rules! Depcrate_baseInputType {
() => {
// Module: crate::base
// Provides: {"InputType"}
// Dependencies: {}
# [doc = " Represents a GraphQL input type."] pub trait InputType : Send + Sync + Sized { # [doc = " The raw type used for validator."] # [doc = ""] # [doc = " Usually it is `Self`, but the wrapper type is its internal type."] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " `i32::RawValueType` is `i32`"] # [doc = " `Option<i32>::RawValueType` is `i32`."] type RawValueType : ? Sized ; # [doc = " Type the name."] fn type_name () -> Cow < 'static , str > ; # [doc = " Qualified typename."] fn qualified_type_name () -> String { format ! ("{}!" , Self :: type_name ()) } # [doc = " Create type information in the registry and return qualified typename."] fn create_type_info (registry : & mut registry :: Registry) -> String ; # [doc = " Parse from `Value`. None represents undefined."] fn parse (value : Option < Value >) -> InputValueResult < Self > ; # [doc = " Convert to a `Value` for introspection."] fn to_value (& self) -> Value ; # [doc = " Get the federation fields, only for InputObject."] # [doc (hidden)] fn federation_fields () -> Option < String > { None } # [doc = " Returns a reference to the raw value."] fn as_raw_value (& self) -> Option < & Self :: RawValueType > ; }
};
}
