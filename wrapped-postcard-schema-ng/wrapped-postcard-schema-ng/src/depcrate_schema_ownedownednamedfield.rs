// Generated macro for OwnedNamedField (struct)
macro_rules! Depcrate_schema_ownedOwnedNamedField {
() => {
// Module: crate::schema::owned
// Provides: {"OwnedNamedField"}
// Dependencies: {}
# [doc = " The owned version of [`NamedField`]"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct OwnedNamedField { # [doc = " The name of this value"] pub name : Box < str > , # [doc = " The type of this value"] pub ty : OwnedDataModelType , }
};
}
