// Generated macro for impl_960 (impl)
macro_rules! Depcrate_util_spanned_valueimpl_960 {
() => {
// Module: crate::util::spanned_value
// Provides: {"impl_960"}
// Dependencies: {}
impl < T : Spanned > From < T > for SpannedValue < T > { fn from (value : T) -> Self { let span = value . span () ; SpannedValue :: new (value , span) } }
};
}
