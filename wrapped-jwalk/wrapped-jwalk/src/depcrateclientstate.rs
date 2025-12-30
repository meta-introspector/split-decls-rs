// Generated macro for ClientState (trait)
macro_rules! DepcrateClientState {
() => {
// Module: crate
// Provides: {"ClientState"}
// Dependencies: {}
# [doc = " Client state maintained while performing walk."] # [doc = ""] # [doc = " for state stored in DirEntry's"] # [doc = " [`client_state`](struct.DirEntry.html#field.client_state) field."] # [doc = ""] # [doc = " Client state can be stored from within the"] # [doc = " [`process_read_dir`](struct.WalkDirGeneric.html#method.process_read_dir) callback."] # [doc = " The type of ClientState is determined by WalkDirGeneric type parameter."] pub trait ClientState : Send + Default + Debug + 'static { # [doc = " The state held on directory level."] type ReadDirState : Clone + Send + Default + Debug + 'static ; # [doc = " The state held for each entry of the directory."] type DirEntryState : Send + Default + Debug + 'static ; }
};
}
