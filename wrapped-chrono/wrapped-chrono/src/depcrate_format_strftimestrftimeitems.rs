// Generated macro for StrftimeItems (struct)
macro_rules! Depcrate_format_strftimeStrftimeItems {
() => {
// Module: crate::format::strftime
// Provides: {"StrftimeItems"}
// Dependencies: {}
# [doc = " Parsing iterator for `strftime`-like format strings."] # [doc = ""] # [doc = " See the [`format::strftime` module](crate::format::strftime) for supported formatting"] # [doc = " specifiers."] # [doc = ""] # [doc = " `StrftimeItems` is used in combination with more low-level methods such as [`format::parse()`]"] # [doc = " or [`format_with_items`]."] # [doc = ""] # [doc = " If formatting or parsing date and time values is not performance-critical, the methods"] # [doc = " [`parse_from_str`] and [`format`] on types such as [`DateTime`](crate::DateTime) are easier to"] # [doc = " use."] # [doc = ""] # [doc = " [`format`]: crate::DateTime::format"] # [doc = " [`format_with_items`]: crate::DateTime::format"] # [doc = " [`parse_from_str`]: crate::DateTime::parse_from_str"] # [doc = " [`DateTime`]: crate::DateTime"] # [doc = " [`format::parse()`]: crate::format::parse()"] # [derive (Clone , Debug)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct StrftimeItems < 'a > { # [doc = " Remaining portion of the string."] remainder : & 'a str , # [doc = " If the current specifier is composed of multiple formatting items (e.g. `%+`),"] # [doc = " `queue` stores a slice of `Item`s that have to be returned one by one."] queue : & 'static [Item < 'static >] , lenient : bool , # [cfg (feature = "unstable-locales")] locale_str : & 'a str , # [cfg (feature = "unstable-locales")] locale : Option < Locale > , }
};
}
