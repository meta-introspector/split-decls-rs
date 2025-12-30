// Generated macro for impl_337 (impl)
macro_rules! Depcrate_format_strftimeimpl_337 {
() => {
// Module: crate::format::strftime
// Provides: {"impl_337"}
// Dependencies: {}
impl < 'a > Iterator for StrftimeItems < 'a > { type Item = Item < 'a > ; fn next (& mut self) -> Option < Item < 'a > > { if let Some ((item , remainder)) = self . queue . split_first () { self . queue = remainder ; return Some (item . clone ()) ; } # [cfg (feature = "unstable-locales")] if ! self . locale_str . is_empty () { let (remainder , item) = self . parse_next_item (self . locale_str) ? ; self . locale_str = remainder ; return Some (item) ; } let (remainder , item) = self . parse_next_item (self . remainder) ? ; self . remainder = remainder ; Some (item) } }
};
}
