// Generated macro for impl_430 (impl)
macro_rules! Depcrate_builder_value_parserimpl_430 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_430"}
// Dependencies: {}
impl ValueParserFactory for i8 { type Parser = RangedI64ValueParser < i8 > ; fn value_parser () -> Self :: Parser { let start : i64 = i8 :: MIN . into () ; let end : i64 = i8 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
};
}
