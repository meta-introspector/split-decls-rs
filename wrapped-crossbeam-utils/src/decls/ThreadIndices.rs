macro_rules! ThreadIndices {
    () => {
        # [doc = " The global registry keeping track of registered threads and indices."] struct ThreadIndices { # [doc = " Mapping from `ThreadId` to thread index."] mapping : HashMap < ThreadId , usize > , # [doc = " A list of free indices."] free_list : Vec < usize > , # [doc = " The next index to allocate if the free list is empty."] next_index : usize , }
    };
}

ThreadIndices!();