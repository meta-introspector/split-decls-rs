// Generated macro for impl_674 (impl)
macro_rules! Depcrate_provider_pattern_runtime_displayimpl_674 {
() => {
// Module: crate::provider::pattern::runtime::display
// Provides: {"impl_674"}
// Dependencies: {}
# [doc = " This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35"] # [doc = " pattern string."] impl Writeable for Pattern < '_ > { fn write_to < W : Write + ? Sized > (& self , formatter : & mut W) -> fmt :: Result { let mut buffer = String :: new () ; for pattern_item in self . items . iter () { match pattern_item { PatternItem :: Field (field) => { dump_buffer_into_formatter (& buffer , formatter) ? ; buffer . clear () ; field . write_to (formatter) ? ; if let FieldSymbol :: DecimalSecond (decimal_second) = field . symbol { formatter . write_char ('.') ? ; for _ in 0 .. (decimal_second as u8) { formatter . write_char ('S') ? ; } } } PatternItem :: Literal (ch) => { buffer . push (ch) ; } } } dump_buffer_into_formatter (& buffer , formatter) ? ; buffer . clear () ; Ok (()) } }
};
}
