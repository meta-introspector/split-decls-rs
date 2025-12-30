// Generated macro for impl_1431 (impl)
macro_rules! Depcrate_stringimpl_1431 {
() => {
// Module: crate::string
// Provides: {"impl_1431"}
// Dependencies: {}
impl StrategyFromRegex for String { type Strategy = RegexGeneratorStrategy < Self > ; fn from_regex (regex : & str) -> Self :: Strategy { string_regex (regex) . unwrap () } }
};
}
