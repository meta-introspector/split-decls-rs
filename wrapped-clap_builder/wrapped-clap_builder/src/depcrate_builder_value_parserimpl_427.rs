// Generated macro for impl_427 (impl)
macro_rules! Depcrate_builder_value_parserimpl_427 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_427"}
// Dependencies: {}
impl ValueParserFactory for Box < std :: path :: Path > { type Parser = MapValueParser < PathBufValueParser , fn (std :: path :: PathBuf) -> Box < std :: path :: Path > > ; fn value_parser () -> Self :: Parser { PathBufValueParser :: new () . map (std :: path :: PathBuf :: into_boxed_path) } }
};
}
