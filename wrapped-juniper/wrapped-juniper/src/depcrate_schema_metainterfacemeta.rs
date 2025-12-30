// Generated macro for InterfaceMeta (struct)
macro_rules! Depcrate_schema_metaInterfaceMeta {
() => {
// Module: crate::schema::meta
// Provides: {"InterfaceMeta"}
// Dependencies: {}
# [doc = " Interface type metadata"] # [derive (Debug)] pub struct InterfaceMeta < S > { # [doc (hidden)] pub name : ArcStr , # [doc (hidden)] pub description : Option < ArcStr > , # [doc (hidden)] pub fields : Vec < Field < S > > , # [doc (hidden)] pub interface_names : Vec < ArcStr > , }
};
}
