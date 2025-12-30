// Generated macro for schema_keys (module)
macro_rules! Depcrate_internals_attributesschema_keys {
() => {
// Module: crate::internals::attributes
// Provides: {"schema_keys"}
// Dependencies: {}
# [cfg (feature = "schema")] pub mod schema_keys { use super :: Symbol ; # [doc = " schema - sub-borsh nested meta, `BorshSchema` context"] pub const SCHEMA : Symbol = Symbol ("schema" , "schema(...)") ; # [doc = " params - sub-schema nested meta, field-level only attribute"] pub const PARAMS : Symbol = Symbol ("params" , "params = ...") ; # [doc = " serialize_with - sub-borsh nested meta, field-level only, `BorshSerialize` context"] # [doc = " with_funcs - sub-schema nested meta, field-level only attribute"] pub const WITH_FUNCS : Symbol = Symbol ("with_funcs" , "with_funcs(...)") ; # [doc = " declaration - sub-with_funcs nested meta, field-level only attribute"] pub const DECLARATION : Symbol = Symbol ("declaration" , "declaration = ...") ; # [doc = " definitions - sub-with_funcs nested meta, field-level only attribute"] pub const DEFINITIONS : Symbol = Symbol ("definitions" , "definitions = ...") ; }
};
}
