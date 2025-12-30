// Generated macro for impl_433 (impl)
macro_rules! Depcrate_builder_value_parserimpl_433 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_433"}
// Dependencies: {}
impl ValueParserFactory for u32 { type Parser = RangedI64ValueParser < u32 > ; fn value_parser () -> Self :: Parser { let start : i64 = u32 :: MIN . into () ; let end : i64 = u32 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
};
}
