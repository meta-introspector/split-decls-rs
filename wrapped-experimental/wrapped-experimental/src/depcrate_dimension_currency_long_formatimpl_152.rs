// Generated macro for impl_152 (impl)
macro_rules! Depcrate_dimension_currency_long_formatimpl_152 {
() => {
// Module: crate::dimension::currency::long_format
// Provides: {"impl_152"}
// Dependencies: {}
impl Writeable for LongFormattedCurrency < '_ > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { let operands = self . value . into () ; let display_name = self . extended . display_names . get (operands , self . plural_rules) ; let pattern = self . patterns . patterns . get (operands , self . plural_rules) ; let formatted_value = self . decimal_formatter . format (self . value) ; let interpolated = pattern . interpolate ((formatted_value , display_name)) ; interpolated . write_to (sink) } }
};
}
