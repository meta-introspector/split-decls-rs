// Generated macro for impl_69 (impl)
macro_rules! Depcrate_dimension_currency_compact_formatimpl_69 {
() => {
// Module: crate::dimension::currency::compact_format
// Provides: {"impl_69"}
// Dependencies: {}
impl Writeable for FormattedCompactCurrency < '_ > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { let config = self . essential . pattern_config_map . get_copied (& self . currency_code . 0 . to_unvalidated ()) . unwrap_or (self . essential . default_pattern_config) ; let placeholder_index = match self . options . width { Width :: Short => config . short_placeholder_value , Width :: Narrow => config . narrow_placeholder_value , } ; let currency_placeholder = match placeholder_index { Some (essentials :: PlaceholderValue :: Index (index)) => self . essential . placeholders . get (index . into ()) . ok_or (core :: fmt :: Error) ? , Some (essentials :: PlaceholderValue :: ISO) | None => self . currency_code . 0 . as_str () , } ; let pattern_selection = match self . options . width { Width :: Short => config . short_pattern_selection , Width :: Narrow => config . narrow_pattern_selection , } ; let pattern = match pattern_selection { essentials :: PatternSelection :: Standard => self . essential . standard_pattern . as_ref () , essentials :: PatternSelection :: StandardAlphaNextToNumber => self . essential . standard_alpha_next_to_number_pattern . as_ref () , } . ok_or (core :: fmt :: Error) ? ; pattern . interpolate ((self . compact_decimal_formatter . format_fixed_decimal (self . value) , currency_placeholder ,)) . write_to (sink) ? ; Ok (()) } }
};
}
