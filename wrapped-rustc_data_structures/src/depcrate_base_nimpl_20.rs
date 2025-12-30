// Generated macro for impl_20 (impl)
macro_rules! Depcrate_base_nimpl_20 {
() => {
// Module: crate::base_n
// Provides: {"impl_20"}
// Dependencies: {}
impl std :: ops :: Deref for BaseNString { type Target = str ; fn deref (& self) -> & str { self . buf [self . start ..] . as_str () } }
};
}
