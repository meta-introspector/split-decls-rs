// Generated macro for OwnedNamedType (struct)
macro_rules! Depcrate_schema_ownedOwnedNamedType {
() => {
// Module: crate::schema::owned
// Provides: {"OwnedNamedType"}
// Dependencies: {}
# [doc = " The owned version of [`NamedType`]"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct OwnedNamedType { # [doc = " The name of this type"] pub name : String , # [doc = " The type"] pub ty : OwnedDataModelType , }
};
}
