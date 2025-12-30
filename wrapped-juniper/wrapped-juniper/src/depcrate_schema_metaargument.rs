// Generated macro for Argument (struct)
macro_rules! Depcrate_schema_metaArgument {
() => {
// Module: crate::schema::meta
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " Metadata for an argument to a field"] # [derive (Debug , Clone)] pub struct Argument < S > { # [doc (hidden)] pub name : ArcStr , # [doc (hidden)] pub description : Option < ArcStr > , # [doc (hidden)] pub arg_type : Type , # [doc (hidden)] pub default_value : Option < InputValue < S > > , # [doc (hidden)] pub deprecation_status : DeprecationStatus , }
};
}
