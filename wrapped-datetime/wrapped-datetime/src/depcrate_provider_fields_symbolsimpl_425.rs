// Generated macro for impl_425 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_425 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_425"}
// Dependencies: {}
impl Weekday { # [doc = " UTS 35 says that \"e\" (local weekday) and \"E\" (format weekday) have the same non-numeric names."] # [doc = ""] # [doc = " This function normalizes \"e\" to \"E\"."] pub (crate) fn to_format_symbol (self) -> Self { match self { Weekday :: Local => Weekday :: Format , other => other , } } }
};
}
