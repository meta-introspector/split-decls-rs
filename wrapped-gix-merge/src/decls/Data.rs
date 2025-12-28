macro_rules! Data {
    () => {
        # [doc = " The data of a mergeable resource, as it could be determined and computed previously."] # [derive (Debug , Copy , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum Data < 'a > { # [doc = " The object is missing, either because it didn't exist in the working tree or because its `id` was null."] # [doc = " Such data equals an empty buffer."] Missing , # [doc = " The textual data as processed and ready for merging, i.e. suitable for storage in Git."] Buffer (& 'a [u8]) , # [doc = " The file or blob is above the big-file threshold and cannot be processed."] # [doc = ""] # [doc = " In this state, the file cannot be merged."] TooLarge { # [doc = " The size of the object prior to performing any filtering or as it was found on disk."] # [doc = ""] # [doc = " Note that technically, the size isn't always representative of the same 'state' of the"] # [doc = " content, as once it can be the size of the blob in Git, and once it's the size of file"] # [doc = " in the worktree."] size : u64 , } , }
    };
}

Data!();