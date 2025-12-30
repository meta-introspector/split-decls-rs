// Generated macro for impl_423 (impl)
macro_rules! Depcrate_builder_value_parserimpl_423 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_423"}
// Dependencies: {}
impl ValueParserFactory for Box < str > { type Parser = MapValueParser < StringValueParser , fn (String) -> Box < str > > ; fn value_parser () -> Self :: Parser { StringValueParser :: new () . map (String :: into_boxed_str) } }
};
}
