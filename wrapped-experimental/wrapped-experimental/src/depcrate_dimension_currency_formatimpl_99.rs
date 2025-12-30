// Generated macro for impl_99 (impl)
macro_rules! Depcrate_dimension_currency_formatimpl_99 {
() => {
// Module: crate::dimension::currency::format
// Provides: {"impl_99"}
// Dependencies: {}
impl Writeable for FormattedCurrency < '_ > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { let config = self . essential . pattern_config_map . get_copied (& self . currency_code . 0 . to_unvalidated ()) . unwrap_or (self . essential . default_pattern_config) ; let placeholder_index = match self . options . width { Width :: Short => config . short_placeholder_value , Width :: Narrow => config . narrow_placeholder_value , } ; let currency_sign_value = match placeholder_index { Some (essentials :: PlaceholderValue :: Index (index)) => self . essential . placeholders . get (index . into ()) . ok_or (core :: fmt :: Error) ? , Some (essentials :: PlaceholderValue :: ISO) | None => self . currency_code . 0 . as_str () , } ; let pattern_selection = match self . options . width { Width :: Short => config . short_pattern_selection , Width :: Narrow => config . narrow_pattern_selection , } ; let pattern = match pattern_selection { essentials :: PatternSelection :: Standard => self . essential . standard_pattern . as_ref () , essentials :: PatternSelection :: StandardAlphaNextToNumber => self . essential . standard_alpha_next_to_number_pattern . as_ref () , } . ok_or (core :: fmt :: Error) ? ; pattern . interpolate ((self . decimal_formatter . format (self . value) , currency_sign_value ,)) . write_to (sink) ? ; Ok (()) } }
};
}
