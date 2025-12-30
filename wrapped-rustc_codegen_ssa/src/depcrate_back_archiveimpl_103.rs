// Generated macro for impl_103 (impl)
macro_rules! Depcrate_back_archiveimpl_103 {
() => {
// Module: crate::back::archive
// Provides: {"impl_103"}
// Dependencies: {}
impl ArchiveBuilderBuilder for ArArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > { Box :: new (ArArchiveBuilder :: new (sess , & DEFAULT_OBJECT_READER)) } }
};
}
