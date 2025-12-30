// Generated macro for impl_2777 (impl)
macro_rules! Depcrate_settingsimpl_2777 {
() => {
// Module: crate::settings
// Provides: {"impl_2777"}
// Dependencies: {}
impl < 'a > From < & 'a dyn TargetIsa > for FlagsOrIsa < 'a > { fn from (isa : & 'a dyn TargetIsa) -> FlagsOrIsa < 'a > { FlagsOrIsa { flags : isa . flags () , isa : Some (isa) , } } }
};
}
