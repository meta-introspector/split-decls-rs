// Generated macro for impl_434 (impl)
macro_rules! Depcrate_builder_value_parserimpl_434 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_434"}
// Dependencies: {}
impl ValueParserFactory for i32 { type Parser = RangedI64ValueParser < i32 > ; fn value_parser () -> Self :: Parser { let start : i64 = i32 :: MIN . into () ; let end : i64 = i32 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
};
}
