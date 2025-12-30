// Generated macro for FormattedRelativeTime (struct)
macro_rules! Depcrate_relativetime_formatFormattedRelativeTime {
() => {
// Module: crate::relativetime::format
// Provides: {"FormattedRelativeTime"}
// Dependencies: {}
# [doc = " An intermediate structure returned by [`RelativeTimeFormatter`](crate::relativetime::RelativeTimeFormatter)."] # [doc = " This structure can be consumed via [`Writeable`](Writeable) trait to a string or buffer."] pub struct FormattedRelativeTime < 'a > { pub (crate) formatter : & 'a RelativeTimeFormatter , pub (crate) options : & 'a RelativeTimeFormatterOptions , pub (crate) value : Decimal , pub (crate) is_negative : bool , }
};
}
