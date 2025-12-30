// Generated macro for impl_438 (impl)
macro_rules! Depcrate_builder_value_parserimpl_438 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_438"}
// Dependencies: {}
impl < T > ValueParserFactory for std :: num :: Wrapping < T > where T : ValueParserFactory , < T as ValueParserFactory > :: Parser : TypedValueParser < Value = T > , T : Send + Sync + Clone , { type Parser = MapValueParser < < T as ValueParserFactory > :: Parser , fn (T) -> std :: num :: Wrapping < T > > ; fn value_parser () -> Self :: Parser { T :: value_parser () . map (std :: num :: Wrapping) } }
};
}
