// Generated macro for OwnedDataModelVariant (enum)
macro_rules! Depcrate_schema_ownedOwnedDataModelVariant {
() => {
// Module: crate::schema::owned
// Provides: {"OwnedDataModelVariant"}
// Dependencies: {}
# [doc = " The owned version of [`DataModelVariant`]"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum OwnedDataModelVariant { # [doc = " The \"unit variant\" Serde Data Model Type"] UnitVariant , # [doc = " The \"newtype variant\" Serde Data Model Type"] NewtypeVariant (Box < OwnedNamedType >) , # [doc = " The \"Tuple Variant\" Serde Data Model Type"] TupleVariant (Vec < OwnedNamedType >) , # [doc = " The \"Struct Variant\" Serde Data Model Type"] StructVariant (Vec < OwnedNamedValue >) , }
};
}
