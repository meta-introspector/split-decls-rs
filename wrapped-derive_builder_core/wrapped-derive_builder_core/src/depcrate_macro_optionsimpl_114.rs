// Generated macro for impl_114 (impl)
macro_rules! Depcrate_macro_optionsimpl_114 {
() => {
// Module: crate::macro_options
// Provides: {"impl_114"}
// Dependencies: {}
impl TryFrom < Vec < Attribute > > for FieldForwardedAttrs { type Error = Error ; fn try_from (value : Vec < Attribute >) -> Result < Self , Self :: Error > { let mut result = Self :: default () ; distribute_and_unnest_attrs (value , & mut [("builder_field_attr" , & mut result . field) , ("builder_setter_attr" , & mut result . setter) ,] ,) ? ; Ok (result) } }
};
}
