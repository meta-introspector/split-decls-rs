// Generated macro for OwnedNamedValue (struct)
macro_rules! Depcrate_schema_ownedOwnedNamedValue {
() => {
// Module: crate::schema::owned
// Provides: {"OwnedNamedValue"}
// Dependencies: {}
# [doc = " The owned version of [`NamedValue`]"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct OwnedNamedValue { # [doc = " The name of this value"] pub name : String , # [doc = " The type of this value"] pub ty : OwnedNamedType , }
};
}
