// Generated macro for impl_429 (impl)
macro_rules! Depcrate_builder_value_parserimpl_429 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_429"}
// Dependencies: {}
impl ValueParserFactory for u8 { type Parser = RangedI64ValueParser < u8 > ; fn value_parser () -> Self :: Parser { let start : i64 = u8 :: MIN . into () ; let end : i64 = u8 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
};
}
