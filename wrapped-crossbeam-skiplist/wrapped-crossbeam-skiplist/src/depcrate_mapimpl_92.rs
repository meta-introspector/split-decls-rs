// Generated macro for impl_92 (impl)
macro_rules! Depcrate_mapimpl_92 {
() => {
// Module: crate::map
// Provides: {"impl_92"}
// Dependencies: {}
impl < K , V > Drop for Entry < '_ , K , V > { fn drop (& mut self) { unsafe { ManuallyDrop :: into_inner (ptr :: read (& self . inner)) . release_with_pin (epoch :: pin) ; } } }
};
}
