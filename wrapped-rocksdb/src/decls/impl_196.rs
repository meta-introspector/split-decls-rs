macro_rules! deps {
    () => {
        CuckooTableOptions!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl CuckooTableOptions { # [doc = " Determines the utilization of hash tables. Smaller values"] # [doc = " result in larger hash tables with fewer collisions."] # [doc = " Default: 0.9"] pub fn set_hash_ratio (& mut self , ratio : f64) { unsafe { ffi :: rocksdb_cuckoo_options_set_hash_ratio (self . inner , ratio) ; } } # [doc = " A property used by builder to determine the depth to go to"] # [doc = " to search for a path to displace elements in case of"] # [doc = " collision. See Builder.MakeSpaceForKey method. Higher"] # [doc = " values result in more efficient hash tables with fewer"] # [doc = " lookups but take more time to build."] # [doc = " Default: 100"] pub fn set_max_search_depth (& mut self , depth : u32) { unsafe { ffi :: rocksdb_cuckoo_options_set_max_search_depth (self . inner , depth) ; } } # [doc = " In case of collision while inserting, the builder"] # [doc = " attempts to insert in the next cuckoo_block_size"] # [doc = " locations before skipping over to the next Cuckoo hash"] # [doc = " function. This makes lookups more cache friendly in case"] # [doc = " of collisions."] # [doc = " Default: 5"] pub fn set_cuckoo_block_size (& mut self , size : u32) { unsafe { ffi :: rocksdb_cuckoo_options_set_cuckoo_block_size (self . inner , size) ; } } # [doc = " If this option is enabled, user key is treated as uint64_t and its value"] # [doc = " is used as hash value directly. This option changes builder's behavior."] # [doc = " Reader ignore this option and behave according to what specified in"] # [doc = " table property."] # [doc = " Default: false"] pub fn set_identity_as_first_hash (& mut self , flag : bool) { unsafe { ffi :: rocksdb_cuckoo_options_set_identity_as_first_hash (self . inner , c_uchar :: from (flag)) ; } } # [doc = " If this option is set to true, module is used during hash calculation."] # [doc = " This often yields better space efficiency at the cost of performance."] # [doc = " If this option is set to false, # of entries in table is constrained to"] # [doc = " be power of two, and bit and is used to calculate hash, which is faster in general."] # [doc = " Default: true"] pub fn set_use_module_hash (& mut self , flag : bool) { unsafe { ffi :: rocksdb_cuckoo_options_set_use_module_hash (self . inner , c_uchar :: from (flag)) ; } } }
    };
}

impl_196!();