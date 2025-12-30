// Generated macro for SchemaTranslator (trait)
macro_rules! Depcrate_schema_translateSchemaTranslator {
() => {
// Module: crate::schema::translate
// Provides: {"SchemaTranslator"}
// Dependencies: {}
# [cfg_attr (not (feature = "schema-language") , expect (dead_code , reason = "common abstraction"))] pub trait SchemaTranslator < 'a , T > { fn translate_schema < S : 'a + ScalarValue > (s : & 'a SchemaType < S >) -> T ; }
};
}
