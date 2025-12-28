macro_rules! ProgressId {
    () => {
        # [doc = " The progress ids used in [`write_from_index_paths()`][multi_index::File::write_from_index_paths()]."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " Counts each path in the input set whose entries we enumerate and write into the multi-index"] FromPathsCollectingEntries , # [doc = " The amount of bytes written as part of the multi-index."] BytesWritten , }
    };
}

ProgressId!();