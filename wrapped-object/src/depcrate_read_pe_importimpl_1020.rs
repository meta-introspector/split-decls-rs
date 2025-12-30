// Generated macro for impl_1020 (impl)
macro_rules! Depcrate_read_pe_importimpl_1020 {
() => {
// Module: crate::read::pe::import
// Provides: {"impl_1020"}
// Dependencies: {}
impl < 'data > Iterator for DelayLoadDescriptorIterator < 'data > { type Item = Result < & 'data pe :: ImageDelayloadDescriptor > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
