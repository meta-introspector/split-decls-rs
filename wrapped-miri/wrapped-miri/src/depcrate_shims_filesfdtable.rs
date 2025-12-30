// Generated macro for FdTable (struct)
macro_rules! Depcrate_shims_filesFdTable {
() => {
// Module: crate::shims::files
// Provides: {"FdTable"}
// Dependencies: {}
# [doc = " The file descriptor table"] # [derive (Debug)] pub struct FdTable { pub fds : BTreeMap < FdNum , DynFileDescriptionRef > , # [doc = " Unique identifier for file description, used to differentiate between various file description."] next_file_description_id : FdId , }
};
}
