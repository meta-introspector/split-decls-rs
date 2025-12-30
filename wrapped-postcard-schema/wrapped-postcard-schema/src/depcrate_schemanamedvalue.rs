// Generated macro for NamedValue (struct)
macro_rules! Depcrate_schemaNamedValue {
() => {
// Module: crate::schema
// Provides: {"NamedValue"}
// Dependencies: {}
# [doc = " This represents a named struct field."] # [doc = ""] # [doc = " For example, in `struct Ex { a: u32 }` the field `a` would be reflected as"] # [doc = " `NamedValue { name: \"a\", ty: DataModelType::U32 }`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct NamedValue { # [doc = " The name of this value"] pub name : & 'static str , # [doc = " The type of this value"] pub ty : & 'static NamedType , }
};
}
