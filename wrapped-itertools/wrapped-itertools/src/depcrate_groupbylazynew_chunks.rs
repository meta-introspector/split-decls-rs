// Generated macro for new_chunks (function)
macro_rules! Depcrate_groupbylazynew_chunks {
() => {
// Module: crate::groupbylazy
// Provides: {"new_chunks"}
// Dependencies: {}
# [doc = " Create a new"] pub fn new_chunks < J > (iter : J , size : usize) -> IntoChunks < J :: IntoIter > where J : IntoIterator , { IntoChunks { inner : RefCell :: new (GroupInner { key : ChunkIndex :: new (size) , iter : iter . into_iter () , current_key : None , current_elt : None , done : false , top_group : 0 , oldest_buffered_group : 0 , bottom_group : 0 , buffer : Vec :: new () , dropped_group : ! 0 , }) , index : Cell :: new (0) , } }
};
}
