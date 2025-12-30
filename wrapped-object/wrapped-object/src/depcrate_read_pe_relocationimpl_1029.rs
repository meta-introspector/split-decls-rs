// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_read_pe_relocationimpl_1029 {
() => {
// Module: crate::read::pe::relocation
// Provides: {"impl_1029"}
// Dependencies: {}
impl < 'data > Iterator for RelocationBlockIterator < 'data > { type Item = Result < RelocationIterator < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
