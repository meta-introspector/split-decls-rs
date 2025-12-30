// Generated macro for impl_674 (impl)
macro_rules! Depcrate_personnames_formatterimpl_674 {
() => {
// Module: crate::personnames::formatter
// Provides: {"impl_674"}
// Dependencies: {}
impl From < & PersonNamesFormatterOptions > for PersonNamesFormattingAttributesMask { fn from (value : & PersonNamesFormatterOptions) -> Self { PersonNamesFormattingAttributes :: from (value . order) . bit_value () | PersonNamesFormattingAttributes :: from (value . length) . bit_value () | PersonNamesFormattingAttributes :: from (value . usage) . bit_value () | PersonNamesFormattingAttributes :: from (value . formality) . bit_value () } }
};
}
