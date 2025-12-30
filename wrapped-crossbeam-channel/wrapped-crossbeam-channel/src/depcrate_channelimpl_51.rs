// Generated macro for impl_51 (impl)
macro_rules! Depcrate_channelimpl_51 {
() => {
// Module: crate::channel
// Provides: {"impl_51"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . receiver . recv () . ok () } }
};
}
