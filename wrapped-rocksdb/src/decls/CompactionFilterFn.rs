macro_rules! deps {
    () => {
        Decision!();
    };
}

macro_rules! CompactionFilterFn {
    () => {
        deps!();
        # [doc = " Function to filter compaction with."] # [doc = ""] # [doc = " This function takes the level of compaction, the key, and the existing value"] # [doc = " and returns the decision about how to handle the Key-Value pair."] # [doc = ""] # [doc = " See [Options::set_compaction_filter][set_compaction_filter] for more details"] # [doc = ""] # [doc = " [set_compaction_filter]: ../struct.Options.html#method.set_compaction_filter"] pub trait CompactionFilterFn : FnMut (u32 , & [u8] , & [u8]) -> Decision { }
    };
}

CompactionFilterFn!();