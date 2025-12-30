// Generated macro for impl_440 (impl)
macro_rules! Depcrate_builder_value_parserimpl_440 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_440"}
// Dependencies: {}
impl < T > ValueParserFactory for std :: sync :: Arc < T > where T : ValueParserFactory , < T as ValueParserFactory > :: Parser : TypedValueParser < Value = T > , T : Send + Sync + Clone , { type Parser = MapValueParser < < T as ValueParserFactory > :: Parser , fn (T) -> std :: sync :: Arc < T > > ; fn value_parser () -> Self :: Parser { T :: value_parser () . map (std :: sync :: Arc :: new) } }
};
}
