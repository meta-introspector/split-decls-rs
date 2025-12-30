// Generated macro for impl_431 (impl)
macro_rules! Depcrate_builder_value_parserimpl_431 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_431"}
// Dependencies: {}
impl ValueParserFactory for u16 { type Parser = RangedI64ValueParser < u16 > ; fn value_parser () -> Self :: Parser { let start : i64 = u16 :: MIN . into () ; let end : i64 = u16 :: MAX . into () ; RangedI64ValueParser :: new () . range (start ..= end) } }
};
}
