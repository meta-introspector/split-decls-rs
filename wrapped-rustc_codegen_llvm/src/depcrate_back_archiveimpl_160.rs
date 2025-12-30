// Generated macro for impl_160 (impl)
macro_rules! Depcrate_back_archiveimpl_160 {
() => {
// Module: crate::back::archive
// Provides: {"impl_160"}
// Dependencies: {}
impl ArchiveBuilderBuilder for LlvmArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > { Box :: new (ArArchiveBuilder :: new (sess , & LLVM_OBJECT_READER)) } }
};
}
