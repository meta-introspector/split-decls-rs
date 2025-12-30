// Generated macro for OwnedData (enum)
macro_rules! Depcrate_schema_ownedOwnedData {
() => {
// Module: crate::schema::owned
// Provides: {"OwnedData"}
// Dependencies: {}
# [doc = " The owned version of [`Data`]."] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum OwnedData { # [doc = " The \"Unit Struct\" or \"Unit Variant\" Serde Data Model Type"] Unit , # [doc = " The \"Newtype Struct\" or \"Newtype Variant\" Serde Data Model Type"] Newtype (Box < OwnedDataModelType >) , # [doc = " The \"Tuple Struct\" or \"Tuple Variant\" Serde Data Model Type"] Tuple (Box < [OwnedDataModelType] >) , # [doc = " The \"Struct\" or \"Struct Variant\" Serde Data Model Type"] Struct (Box < [OwnedNamedField] >) , }
};
}
