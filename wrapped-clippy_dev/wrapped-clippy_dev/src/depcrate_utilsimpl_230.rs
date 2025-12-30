// Generated macro for impl_230 (impl)
macro_rules! Depcrate_utilsimpl_230 {
() => {
// Module: crate::utils
// Provides: {"impl_230"}
// Dependencies: {}
impl UpdateStatus { # [must_use] pub fn from_changed (value : bool) -> Self { if value { Self :: Changed } else { Self :: Unchanged } } # [must_use] pub fn is_changed (self) -> bool { matches ! (self , Self :: Changed) } }
};
}
