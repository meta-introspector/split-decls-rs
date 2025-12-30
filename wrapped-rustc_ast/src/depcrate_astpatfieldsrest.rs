// Generated macro for PatFieldsRest (enum)
macro_rules! Depcrate_astPatFieldsRest {
() => {
// Module: crate::ast
// Provides: {"PatFieldsRest"}
// Dependencies: {}
# [doc = " Whether the `..` is present in a struct fields pattern."] # [derive (Clone , Copy , Encodable , Decodable , Debug , PartialEq , Walkable)] pub enum PatFieldsRest { # [doc = " `module::StructName { field, ..}`"] Rest (Span) , # [doc = " `module::StructName { field, syntax error }`"] Recovered (ErrorGuaranteed) , # [doc = " `module::StructName { field }`"] None , }
};
}
