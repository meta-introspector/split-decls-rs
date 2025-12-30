// Generated macro for NamedVariant (struct)
macro_rules! Depcrate_schemaNamedVariant {
() => {
// Module: crate::schema
// Provides: {"NamedVariant"}
// Dependencies: {}
# [doc = " An enum variant with a name, e.g. `T::Bar(...)`"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct NamedVariant { # [doc = " The name of this variant"] pub name : & 'static str , # [doc = " The type of this variant"] pub ty : & 'static DataModelVariant , }
};
}
