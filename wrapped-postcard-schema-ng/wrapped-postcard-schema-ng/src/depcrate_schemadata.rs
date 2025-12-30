// Generated macro for Data (enum)
macro_rules! Depcrate_schemaData {
() => {
// Module: crate::schema
// Provides: {"Data"}
// Dependencies: {}
# [doc = " The contents of a struct or enum variant."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub enum Data { # [doc = " The \"Unit Struct\" or \"Unit Variant\" Serde Data Model Type"] Unit , # [doc = " The \"Newtype Struct\" or \"Newtype Variant\" Serde Data Model Type"] Newtype (& 'static DataModelType) , # [doc = " The \"Tuple Struct\" or \"Tuple Variant\" Serde Data Model Type"] Tuple (& 'static [& 'static DataModelType]) , # [doc = " The \"Struct\" or \"Struct Variant\" Serde Data Model Type"] Struct (& 'static [& 'static NamedField]) , }
};
}
