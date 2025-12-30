// Generated macro for WalkDirOptions (struct)
macro_rules! DepcrateWalkDirOptions {
() => {
// Module: crate
// Provides: {"WalkDirOptions"}
// Dependencies: {}
struct WalkDirOptions < C : ClientState > { sort : bool , min_depth : usize , max_depth : usize , skip_hidden : bool , follow_links : bool , parallelism : Parallelism , root_read_dir_state : C :: ReadDirState , process_read_dir : Option < Arc < ProcessReadDirFunction < C > > > , }
};
}
