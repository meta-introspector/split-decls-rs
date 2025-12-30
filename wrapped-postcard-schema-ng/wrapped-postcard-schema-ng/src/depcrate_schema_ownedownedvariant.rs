// Generated macro for OwnedVariant (struct)
macro_rules! Depcrate_schema_ownedOwnedVariant {
() => {
// Module: crate::schema::owned
// Provides: {"OwnedVariant"}
// Dependencies: {}
# [doc = " The owned version of [`Variant`]"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct OwnedVariant { # [doc = " The name of this variant"] pub name : Box < str > , # [doc = " The data contained in this variant"] pub data : OwnedData , }
};
}
