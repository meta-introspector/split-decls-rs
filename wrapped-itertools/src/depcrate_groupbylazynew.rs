// Generated macro for new (function)
macro_rules! Depcrate_groupbylazynew {
() => {
// Module: crate::groupbylazy
// Provides: {"new"}
// Dependencies: {}
# [doc = " Create a new"] pub fn new < K , J , F > (iter : J , f : F) -> ChunkBy < K , J :: IntoIter , F > where J : IntoIterator , F : FnMut (& J :: Item) -> K , { ChunkBy { inner : RefCell :: new (GroupInner { key : f , iter : iter . into_iter () , current_key : None , current_elt : None , done : false , top_group : 0 , oldest_buffered_group : 0 , bottom_group : 0 , buffer : Vec :: new () , dropped_group : ! 0 , }) , index : Cell :: new (0) , } }
};
}
