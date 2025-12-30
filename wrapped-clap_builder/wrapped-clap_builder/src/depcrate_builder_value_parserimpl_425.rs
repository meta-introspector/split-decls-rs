// Generated macro for impl_425 (impl)
macro_rules! Depcrate_builder_value_parserimpl_425 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_425"}
// Dependencies: {}
impl ValueParserFactory for Box < std :: ffi :: OsStr > { type Parser = MapValueParser < OsStringValueParser , fn (std :: ffi :: OsString) -> Box < std :: ffi :: OsStr > > ; fn value_parser () -> Self :: Parser { OsStringValueParser :: new () . map (std :: ffi :: OsString :: into_boxed_os_str) } }
};
}
