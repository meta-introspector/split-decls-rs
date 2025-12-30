// Generated macro for impl_122 (impl)
macro_rules! Depcrate_macro_optionsimpl_122 {
() => {
// Module: crate::macro_options
// Provides: {"impl_122"}
// Dependencies: {}
impl TryFrom < Vec < Attribute > > for StructForwardedAttrs { type Error = Error ; fn try_from (value : Vec < Attribute >) -> Result < Self , Self :: Error > { let mut result = Self :: default () ; distribute_and_unnest_attrs (value , & mut [("builder_struct_attr" , & mut result . struct_attrs) , ("builder_impl_attr" , & mut result . impl_attrs) ,] ,) ? ; Ok (result) } }
};
}
