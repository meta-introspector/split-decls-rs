macro_rules! PackIndex {
    () => {
        # [doc = " An index into our [`File::index_names()`] array yielding the name of the index and by implication, its pack file."] pub type PackIndex = u32 ;
    };
}

PackIndex!();