// Generated macro for impl_127 (impl)
macro_rules! Depcrate_dimension_currency_long_compact_formatimpl_127 {
() => {
// Module: crate::dimension::currency::long_compact_format
// Provides: {"impl_127"}
// Dependencies: {}
impl Writeable for FormattedLongCompactCurrency < '_ > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { let operands = self . signed_fixed_decimal . into () ; let display_name = self . extended . display_names . get (operands , self . plural_rules) ; let pattern = self . patterns . patterns . get (operands , self . plural_rules) ; let formatted_value = self . compact_decimal_formatter . format_fixed_decimal (self . signed_fixed_decimal) ; let interpolated = pattern . interpolate ((formatted_value , display_name)) ; interpolated . write_to (sink) } }
};
}
