// Generated macro for Field (struct)
macro_rules! Depcrate_schema_metaField {
() => {
// Module: crate::schema::meta
// Provides: {"Field"}
// Dependencies: {}
# [doc = " Metadata for a field"] # [derive (Debug , Clone)] pub struct Field < S > { # [doc (hidden)] pub name : ArcStr , # [doc (hidden)] pub description : Option < ArcStr > , # [doc (hidden)] pub arguments : Option < Vec < Argument < S > > > , # [doc (hidden)] pub field_type : Type , # [doc (hidden)] pub deprecation_status : DeprecationStatus , }
};
}
