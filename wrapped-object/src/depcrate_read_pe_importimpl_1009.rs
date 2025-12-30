// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_read_pe_importimpl_1009 {
() => {
// Module: crate::read::pe::import
// Provides: {"impl_1009"}
// Dependencies: {}
impl < 'data > Iterator for ImportDescriptorIterator < 'data > { type Item = Result < & 'data pe :: ImageImportDescriptor > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
