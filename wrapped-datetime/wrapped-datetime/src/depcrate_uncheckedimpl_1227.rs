// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_uncheckedimpl_1227 {
() => {
// Module: crate::unchecked
// Provides: {"impl_1227"}
// Dependencies: {}
impl TryWriteable for FormattedDateTimeUnchecked < '_ > { type Error = FormattedDateTimeUncheckedError ; fn try_write_to_parts < S : writeable :: PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { let err = match try_write_pattern_items (self . pattern . metadata () , self . pattern . iter_items () , & self . input , & self . names , self . names . decimal_formatter , sink ,) { Ok (Ok (())) => return Ok (Ok (())) , Err (fmt :: Error) => return Err (fmt :: Error) , Ok (Err (err)) => err , } ; Ok (Err (match err { FormattedDateTimePatternError :: InvalidMonthCode (month_code) => { Self :: Error :: InvalidMonthCode (month_code) } FormattedDateTimePatternError :: InvalidEra (tiny_ascii_str) => { Self :: Error :: InvalidEra (tiny_ascii_str) } FormattedDateTimePatternError :: InvalidCyclicYear { value , max } => { Self :: Error :: InvalidCyclicYear { value , max } } FormattedDateTimePatternError :: DecimalFormatterNotLoaded => { Self :: Error :: DecimalFormatterNotLoaded } FormattedDateTimePatternError :: NamesNotLoaded (error_field) => { Self :: Error :: NamesNotLoaded (error_field) } FormattedDateTimePatternError :: MissingInputField (name) => { Self :: Error :: MissingInputField (name) } FormattedDateTimePatternError :: UnsupportedLength (error_field) => { Self :: Error :: UnsupportedLength (error_field) } FormattedDateTimePatternError :: UnsupportedField (error_field) => { Self :: Error :: UnsupportedField (error_field) } })) } }
};
}
