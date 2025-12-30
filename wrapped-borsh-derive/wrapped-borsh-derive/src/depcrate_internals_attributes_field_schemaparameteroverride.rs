// Generated macro for ParameterOverride (struct)
macro_rules! Depcrate_internals_attributes_field_schemaParameterOverride {
() => {
// Module: crate::internals::attributes::field::schema
// Provides: {"ParameterOverride"}
// Dependencies: {}
# [doc = "\nStruct describes an entry like `order_param => override_type`,  e.g. `K => <K as TraitName>::Associated`\n"] # [derive (Clone)] pub struct ParameterOverride { pub order_param : Ident , arrow_token : Token ! [=>] , pub override_type : Type , }
};
}
