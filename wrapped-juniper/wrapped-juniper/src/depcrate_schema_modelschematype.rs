// Generated macro for SchemaType (struct)
macro_rules! Depcrate_schema_modelSchemaType {
() => {
// Module: crate::schema::model
// Provides: {"SchemaType"}
// Dependencies: {}
# [doc = " Metadata for a schema"] # [derive (Debug)] pub struct SchemaType < S > { pub (crate) description : Option < ArcStr > , pub (crate) types : FnvHashMap < Name , MetaType < S > > , pub (crate) query_type_name : String , pub (crate) mutation_type_name : Option < String > , pub (crate) subscription_type_name : Option < String > , directives : FnvHashMap < ArcStr , DirectiveType < S > > , }
};
}
