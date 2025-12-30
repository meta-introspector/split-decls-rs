// Generated macro for impl_26 (impl)
macro_rules! Depcrate_dataimpl_26 {
() => {
// Module: crate::data
// Provides: {"impl_26"}
// Dependencies: {}
impl From < & Locale > for DataLocale { fn from (locale : & Locale) -> Self { let mut r = Self :: from (& locale . id) ; r . subdivision = locale . extensions . unicode . keywords . get (& unicode_ext :: key ! ("sd")) . and_then (| v | v . as_single_subtag () . copied ()) ; r } }
};
}
