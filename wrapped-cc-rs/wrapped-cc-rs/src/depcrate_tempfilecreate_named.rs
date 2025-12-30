// Generated macro for create_named (function)
macro_rules! Depcrate_tempfilecreate_named {
() => {
// Module: crate::tempfile
// Provides: {"create_named"}
// Dependencies: {}
fn create_named (path : & Path) -> io :: Result < File > { let mut open_options = OpenOptions :: new () ; open_options . read (true) . write (true) . create_new (true) ; # [cfg (all (unix , not (target_os = "wasi")))] < OpenOptions as os :: unix :: fs :: OpenOptionsExt > :: mode (& mut open_options , 0o600) ; # [cfg (windows)] < OpenOptions as os :: windows :: fs :: OpenOptionsExt > :: custom_flags (& mut open_options , :: find_msvc_tools :: windows_sys :: FILE_ATTRIBUTE_TEMPORARY ,) ; open_options . open (path) }
};
}
