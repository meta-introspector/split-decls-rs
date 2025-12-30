// Generated macro for impl_31 (impl)
macro_rules! Depcrate_traitsimpl_31 {
() => {
// Module: crate::traits
// Provides: {"impl_31"}
// Dependencies: {}
impl < F , T : ConvertTryFrom < F > > ConvertTryInto < T > for F { type Error = T :: Error ; fn convert_try_into (self) -> Result < T , T :: Error > { T :: convert_try_from (self) } }
};
}
