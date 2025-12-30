// Generated macro for BaseType (enum)
macro_rules! Depcrate_typesBaseType {
() => {
// Module: crate::types
// Provides: {"BaseType"}
// Dependencies: {}
# [doc = " A GraphQL base type, for example `String` or `[String!]`. This does not"] # [doc = " include whether the type is nullable; for that see [Type](struct.Type.html)."] # [derive (Debug , PartialEq , Eq , Clone , Serialize , Deserialize)] pub enum BaseType { # [doc = " A named type, such as `String`."] Named (Name) , # [doc = " A list type, such as `[String]`."] List (Box < Type >) , }
};
}
