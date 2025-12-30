// Generated macro for ParserNumber (enum)
macro_rules! Depcrate_deParserNumber {
() => {
// Module: crate::de
// Provides: {"ParserNumber"}
// Dependencies: {}
pub (crate) enum ParserNumber { F64 (f64) , U64 (u64) , I64 (i64) , # [cfg (feature = "arbitrary_precision")] String (String) , }
};
}
