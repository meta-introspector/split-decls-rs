// Generated macro for ExecutableDocument (struct)
macro_rules! Depcrate_types_executableExecutableDocument {
() => {
// Module: crate::types::executable
// Provides: {"ExecutableDocument"}
// Dependencies: {}
# [doc = " An executable GraphQL file or request string."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#ExecutableDocument)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ExecutableDocument { # [doc = " The operations of the document."] pub operations : DocumentOperations , # [doc = " The fragments of the document."] pub fragments : HashMap < Name , Positioned < FragmentDefinition > > , }
};
}
