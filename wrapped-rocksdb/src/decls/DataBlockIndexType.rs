macro_rules! DataBlockIndexType {
    () => {
        # [doc = " Used by BlockBasedOptions::set_data_block_index_type."] # [repr (C)] pub enum DataBlockIndexType { # [doc = " Use binary search when performing point lookup for keys in data blocks."] # [doc = " This is the default."] BinarySearch = 0 , # [doc = " Appends a compact hash table to the end of the data block for efficient indexing. Backwards"] # [doc = " compatible with databases created without this feature. Once turned on, existing data will"] # [doc = " be gradually converted to the hash index format."] BinaryAndHash = 1 , }
    };
}

DataBlockIndexType!();