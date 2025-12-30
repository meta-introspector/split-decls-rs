// Generated macro for DataModelVariant (enum)
macro_rules! Depcrate_schemaDataModelVariant {
() => {
// Module: crate::schema
// Provides: {"DataModelVariant"}
// Dependencies: {}
# [doc = " This is similar to [`DataModelType`], however it only contains the potential Data Model Types"] # [doc = " used as variants of an `enum`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub enum DataModelVariant { # [doc = " The \"unit variant\" Serde Data Model Type"] UnitVariant , # [doc = " The \"newtype variant\" Serde Data Model Type"] NewtypeVariant (& 'static NamedType) , # [doc = " The \"Tuple Variant\" Serde Data Model Type"] TupleVariant (& 'static [& 'static NamedType]) , # [doc = " The \"Struct Variant\" Serde Data Model Type"] StructVariant (& 'static [& 'static NamedValue]) , }
};
}
