macro_rules! deps {
    () => {
        GroupInner!();
        IntoChunks!();
        ChunkIndex!();
    };
}

macro_rules! new_chunks {
    () => {
        deps!();
        # [doc = " Create a new"] pub fn new_chunks < J > (iter : J , size : usize) -> IntoChunks < J :: IntoIter > where J : IntoIterator , { IntoChunks { inner : RefCell :: new (GroupInner { key : ChunkIndex :: new (size) , iter : iter . into_iter () , current_key : None , current_elt : None , done : false , top_group : 0 , oldest_buffered_group : 0 , bottom_group : 0 , buffer : Vec :: new () , dropped_group : ! 0 , }) , index : Cell :: new (0) , } }
    };
}

new_chunks!()