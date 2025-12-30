// Generated macro for impl_439 (impl)
macro_rules! Depcrate_builder_value_parserimpl_439 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_439"}
// Dependencies: {}
impl < T > ValueParserFactory for Box < T > where T : ValueParserFactory , < T as ValueParserFactory > :: Parser : TypedValueParser < Value = T > , T : Send + Sync + Clone , { type Parser = MapValueParser < < T as ValueParserFactory > :: Parser , fn (T) -> Box < T > > ; fn value_parser () -> Self :: Parser { T :: value_parser () . map (Box :: new) } }
};
}
