// Generated macro for impl_432 (impl)
macro_rules! Depcrate_builder_value_parserimpl_432 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_432"}
// Dependencies: {}
impl ValueParserFactory for i16 { type Parser = RangedI64ValueParser < i16 > ; fn value_parser () -> Self :: Parser { let start : i64 = i16 :: MIN . into () ; let end : i64 = i16 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
};
}
