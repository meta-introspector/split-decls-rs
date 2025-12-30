// Generated macro for impl_44 (impl)
macro_rules! Depcrate_channelimpl_44 {
() => {
// Module: crate::channel
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > Iterator for Iter < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . receiver . recv () . ok () } }
};
}
