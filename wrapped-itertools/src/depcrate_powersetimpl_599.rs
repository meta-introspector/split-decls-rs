// Generated macro for impl_599 (impl)
macro_rules! Depcrate_powersetimpl_599 {
() => {
// Module: crate::powerset
// Provides: {"impl_599"}
// Dependencies: {}
impl < I : Iterator > Powerset < I > { # [doc = " Returns true if `k` has been incremented, false otherwise."] fn increment_k (& mut self) -> bool { if self . combs . k () < self . combs . n () || self . combs . k () == 0 { self . combs . reset (self . combs . k () + 1) ; true } else { false } } }
};
}
