// Generated macro for WalkDirGeneric (struct)
macro_rules! DepcrateWalkDirGeneric {
() => {
// Module: crate
// Provides: {"WalkDirGeneric"}
// Dependencies: {}
# [doc = " Generic builder for walking a directory."] # [doc = ""] # [doc = " [`ClientState`](trait.ClientState.html) type parameter allows you to specify"] # [doc = " state to be stored with each DirEntry from within the"] # [doc = " [`process_read_dir`](struct.WalkDirGeneric.html#method.process_read_dir)"] # [doc = " callback."] # [doc = ""] # [doc = " Use [`WalkDir`](type.WalkDir.html) if you don't need to store client state"] # [doc = " into yeilded DirEntries."] pub struct WalkDirGeneric < C : ClientState > { root : PathBuf , options : WalkDirOptions < C > , }
};
}
