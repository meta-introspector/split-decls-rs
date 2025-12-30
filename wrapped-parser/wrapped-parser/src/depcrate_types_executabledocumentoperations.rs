// Generated macro for DocumentOperations (enum)
macro_rules! Depcrate_types_executableDocumentOperations {
() => {
// Module: crate::types::executable
// Provides: {"DocumentOperations"}
// Dependencies: {}
# [doc = " The operations of a GraphQL document."] # [doc = ""] # [doc = " There is either one anonymous operation or many named operations."] # [derive (Debug , Clone , Serialize , Deserialize)] pub enum DocumentOperations { # [doc = " The document contains a single anonymous operation."] Single (Positioned < OperationDefinition >) , # [doc = " The document contains many named operations."] Multiple (HashMap < Name , Positioned < OperationDefinition > >) , }
};
}
