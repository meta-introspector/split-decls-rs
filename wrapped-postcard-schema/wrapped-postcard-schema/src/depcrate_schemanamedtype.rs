// Generated macro for NamedType (struct)
macro_rules! Depcrate_schemaNamedType {
() => {
// Module: crate::schema
// Provides: {"NamedType"}
// Dependencies: {}
# [doc = " A \"NamedType\" is used to describe the schema of a given type."] # [doc = ""] # [doc = " It contains two pieces of information:"] # [doc = ""] # [doc = " * A `name`, which is the name of the type, e.g. \"u8\" for [`u8`]."] # [doc = " * A `ty`, which is one of the possible [`DataModelType`]s any given type can be represented as."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct NamedType { # [doc = " The name of this type"] pub name : & 'static str , # [doc = " The type"] pub ty : & 'static DataModelType , }
};
}
