// Generated macro for impl_474 (impl)
macro_rules! Depcrate_schema_translate_graphql_parserimpl_474 {
() => {
// Module: crate::schema::translate::graphql_parser
// Provides: {"impl_474"}
// Dependencies: {}
impl < 'a , T > SchemaTranslator < 'a , schema :: Document < 'a , T > > for GraphQLParserTranslator where T : schema :: Text < 'a > + Default , { fn translate_schema < S > (input : & 'a SchemaType < S >) -> schema :: Document < 'a , T > where S : ScalarValue + 'a , { let mut doc = schema :: Document :: default () ; let mut types = input . types . iter () . filter (| (_ , meta) | ! meta . is_builtin ()) . map (| (_ , meta) | GraphQLParserTranslator :: translate_meta (meta)) . map (schema :: Definition :: TypeDefinition) . collect () ; doc . definitions . append (& mut types) ; doc . definitions . push (schema :: Definition :: SchemaDefinition (schema :: SchemaDefinition { position : Pos :: default () , directives : vec ! [] , query : Some (input . query_type_name . as_str () . into ()) , mutation : input . mutation_type_name . as_ref () . map (| s | s . as_str () . into ()) , subscription : input . subscription_type_name . as_ref () . map (| s | s . as_str () . into ()) , } ,)) ; doc } }
};
}
