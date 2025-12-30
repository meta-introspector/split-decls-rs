// Generated macro for impl_37 (impl)
macro_rules! Depcrate_internals_attributes_field_schemaimpl_37 {
() => {
// Module: crate::internals::attributes::field::schema
// Provides: {"impl_37"}
// Dependencies: {}
impl Parse for ParameterOverride { fn parse (input : ParseStream) -> Result < Self , syn :: Error > { Ok (ParameterOverride { order_param : input . parse () ? , arrow_token : input . parse () ? , override_type : input . parse () ? , }) } }
};
}
