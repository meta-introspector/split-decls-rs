// Generated macro for impl_253 (impl)
macro_rules! Depcrate_pattern_formatterimpl_253 {
() => {
// Module: crate::pattern::formatter
// Provides: {"impl_253"}
// Dependencies: {}
impl TryWriteable for FormattedDateTimePattern < '_ > { type Error = FormattedDateTimePatternError ; fn try_write_to_parts < S : writeable :: PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { try_write_pattern_items (self . pattern . 0 . as_borrowed () . metadata , self . pattern . 0 . as_borrowed () . items . iter () , & self . input , & self . names , self . names . decimal_formatter , sink ,) } }
};
}
