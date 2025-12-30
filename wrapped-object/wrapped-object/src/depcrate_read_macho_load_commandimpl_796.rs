// Generated macro for impl_796 (impl)
macro_rules! Depcrate_read_macho_load_commandimpl_796 {
() => {
// Module: crate::read::macho::load_command
// Provides: {"impl_796"}
// Dependencies: {}
impl < 'data , E : Endian > Iterator for LoadCommandIterator < 'data , E > { type Item = Result < LoadCommandData < 'data , E > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
