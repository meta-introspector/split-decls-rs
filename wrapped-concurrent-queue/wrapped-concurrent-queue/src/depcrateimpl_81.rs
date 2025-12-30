// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > Iterator for TryIter < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . queue . pop () . ok () } }
};
}
