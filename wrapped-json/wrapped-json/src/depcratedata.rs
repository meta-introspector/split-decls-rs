// Generated macro for Data (enum)
macro_rules! DepcrateData {
() => {
// Module: crate
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Content of a syntax tree data structure."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum Data { # [doc = " This is an opaque type with no publicly accessible structure."] Private , # [doc = " This type is a braced struct with named fields."] # [cfg_attr (feature = "serde" , serde (rename = "fields"))] Struct (Fields) , # [doc = " This type is an enum."] # [cfg_attr (feature = "serde" , serde (rename = "variants"))] Enum (Variants) , }
};
}
