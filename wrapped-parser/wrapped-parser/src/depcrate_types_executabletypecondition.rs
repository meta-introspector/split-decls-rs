// Generated macro for TypeCondition (struct)
macro_rules! Depcrate_types_executableTypeCondition {
() => {
// Module: crate::types::executable
// Provides: {"TypeCondition"}
// Dependencies: {}
# [doc = " A type a fragment can apply to (`on` followed by the type)."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#TypeCondition)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TypeCondition { # [doc = " The type this fragment applies to."] pub on : Positioned < Name > , }
};
}
