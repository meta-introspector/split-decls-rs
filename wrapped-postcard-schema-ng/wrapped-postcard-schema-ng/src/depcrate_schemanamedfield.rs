// Generated macro for NamedField (struct)
macro_rules! Depcrate_schemaNamedField {
() => {
// Module: crate::schema
// Provides: {"NamedField"}
// Dependencies: {}
# [doc = " This represents a named struct field."] # [doc = ""] # [doc = " For example, in `struct Ex { a: u32 }` the field `a` would be reflected as"] # [doc = " `NamedField { name: \"a\", ty: DataModelType::U32 }`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct NamedField { # [doc = " The name of this field"] pub name : & 'static str , # [doc = " The type of this field"] pub ty : & 'static DataModelType , }
};
}
