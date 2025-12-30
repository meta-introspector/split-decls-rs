// Generated macro for impl_473 (impl)
macro_rules! Depcrate_schema_translate_graphql_parserimpl_473 {
() => {
// Module: crate::schema::translate::graphql_parser
// Provides: {"impl_473"}
// Dependencies: {}
impl < 'a , S : 'a , T > From < & 'a SchemaType < S > > for schema :: Document < 'a , T > where S : ScalarValue , T : schema :: Text < 'a > + Default , { fn from (input : & 'a SchemaType < S >) -> schema :: Document < 'a , T > { GraphQLParserTranslator :: translate_schema (input) } }
};
}
