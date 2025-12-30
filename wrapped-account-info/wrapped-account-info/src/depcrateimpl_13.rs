// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a , T : IntoAccountInfo < 'a > > From < T > for AccountInfo < 'a > { fn from (src : T) -> Self { src . into_account_info () } }
};
}
