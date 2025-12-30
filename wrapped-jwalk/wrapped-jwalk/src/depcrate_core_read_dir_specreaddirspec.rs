// Generated macro for ReadDirSpec (struct)
macro_rules! Depcrate_core_read_dir_specReadDirSpec {
() => {
// Module: crate::core::read_dir_spec
// Provides: {"ReadDirSpec"}
// Dependencies: {}
# [doc = " Specification for reading a directory."] # [doc = ""] # [doc = " When a directory is read a new `ReadDirSpec` is created for each folder"] # [doc = " found in that directory. These specs are then sent to a work queue that is"] # [doc = " used to schedule future directory reads. Use"] # [doc = " [`max_depth`](struct.WalkDir.html#method.max_depth) and"] # [doc = " [`process_read_dir`](struct.WalkDir.html#method.process_read_dir) to change"] # [doc = " this default behavior."] # [derive (Debug)] pub struct ReadDirSpec < C : ClientState > { # [doc = " Depth of the directory to read relative to root of walk."] pub depth : usize , # [doc = " Path of the the directory to read."] pub path : Arc < Path > , # [doc = " Client branch state that was set in the"] # [doc = " [`process_read_dir`](struct.WalkDir.html#method.process_read_dir) callback"] # [doc = " when reading this directory's parent. One intended use case is to store"] # [doc = " `.gitignore` state to filter entries during the walk."] pub client_read_state : C :: ReadDirState , pub (crate) follow_link_ancestors : Arc < Vec < Arc < Path > > > , }
};
}
