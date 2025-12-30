// Generated macro for ReadDirIter (enum)
macro_rules! Depcrate_core_read_dir_iterReadDirIter {
() => {
// Module: crate::core::read_dir_iter
// Provides: {"ReadDirIter"}
// Dependencies: {}
# [doc = " Result<ReadDir> Iterator."] # [doc = ""] # [doc = " Yields ReadDirs (results of fs::read_dir) in order required for recursive"] # [doc = " directory traversal. Depending on Walk/ParWalk state these reads might be"] # [doc = " computed in parallel."] pub enum ReadDirIter < C : ClientState > { Walk { read_dir_spec_stack : Vec < ReadDirSpec < C > > , core_read_dir_callback : Arc < ReadDirCallback < C > > , } , ParWalk { read_dir_result_iter : OrderedQueueIter < Result < ReadDir < C > > > , } , }
};
}
