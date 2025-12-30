// Generated macro for impl_47 (impl)
macro_rules! Depcrate_channelimpl_47 {
() => {
// Module: crate::channel
// Provides: {"impl_47"}
// Dependencies: {}
impl < T > Iterator for TryIter < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . receiver . try_recv () . ok () } }
};
}
