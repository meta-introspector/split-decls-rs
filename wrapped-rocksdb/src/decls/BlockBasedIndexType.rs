macro_rules! BlockBasedIndexType {
    () => {
        # [doc = " Used by BlockBasedOptions::set_index_type."] pub enum BlockBasedIndexType { # [doc = " A space efficient index block that is optimized for"] # [doc = " binary-search-based index."] BinarySearch , # [doc = " The hash index, if enabled, will perform a hash lookup if"] # [doc = " a prefix extractor has been provided through Options::set_prefix_extractor."] HashSearch , # [doc = " A two-level index implementation. Both levels are binary search indexes."] TwoLevelIndexSearch , }
    };
}

BlockBasedIndexType!()