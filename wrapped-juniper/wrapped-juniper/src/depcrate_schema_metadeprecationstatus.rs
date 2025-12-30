// Generated macro for DeprecationStatus (enum)
macro_rules! Depcrate_schema_metaDeprecationStatus {
() => {
// Module: crate::schema::meta
// Provides: {"DeprecationStatus"}
// Dependencies: {}
# [doc = " Whether an item is deprecated, with context."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub enum DeprecationStatus { # [doc = " The field/variant is not deprecated."] Current , # [doc = " The field/variant is deprecated, with an optional reason"] Deprecated (Option < ArcStr >) , }
};
}
