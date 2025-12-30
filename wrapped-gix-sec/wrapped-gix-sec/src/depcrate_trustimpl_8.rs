// Generated macro for impl_8 (impl)
macro_rules! Depcrate_trustimpl_8 {
() => {
// Module: crate::trust
// Provides: {"impl_8"}
// Dependencies: {}
impl < T > Default for Mapping < T > where T : DefaultForLevel , { fn default () -> Self { Mapping { full : T :: default_for_level (Trust :: Full) , reduced : T :: default_for_level (Trust :: Reduced) , } } }
};
}
