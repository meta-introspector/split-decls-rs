// Generated macro for impl_232 (impl)
macro_rules! Depcrate_utilsimpl_232 {
() => {
// Module: crate::utils
// Provides: {"impl_232"}
// Dependencies: {}
impl UpdateMode { # [must_use] pub fn from_check (check : bool) -> Self { if check { Self :: Check } else { Self :: Change } } # [must_use] pub fn is_check (self) -> bool { matches ! (self , Self :: Check) } }
};
}
