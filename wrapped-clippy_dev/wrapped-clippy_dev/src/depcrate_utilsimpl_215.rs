// Generated macro for impl_215 (impl)
macro_rules! Depcrate_utilsimpl_215 {
() => {
// Module: crate::utils
// Provides: {"impl_215"}
// Dependencies: {}
impl UpdateMode { # [must_use] pub fn from_check (check : bool) -> Self { if check { Self :: Check } else { Self :: Change } } # [must_use] pub fn is_check (self) -> bool { matches ! (self , Self :: Check) } }
};
}
