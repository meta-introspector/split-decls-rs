macro_rules! deps {
    () => {
        KeyEncodingType!();
    };
}

macro_rules! PlainTableFactoryOptions {
    () => {
        deps!();
        # [doc = " Used with DBOptions::set_plain_table_factory."] # [doc = " See official [wiki](https://github.com/facebook/rocksdb/wiki/PlainTable-Format) for more"] # [doc = " information."] # [doc = ""] # [doc = " Defaults:"] # [doc = "  user_key_length: 0 (variable length)"] # [doc = "  bloom_bits_per_key: 10"] # [doc = "  hash_table_ratio: 0.75"] # [doc = "  index_sparseness: 16"] # [doc = "  huge_page_tlb_size: 0"] # [doc = "  encoding_type: KeyEncodingType::Plain"] # [doc = "  full_scan_mode: false"] # [doc = "  store_index_in_file: false"] pub struct PlainTableFactoryOptions { pub user_key_length : u32 , pub bloom_bits_per_key : i32 , pub hash_table_ratio : f64 , pub index_sparseness : usize , pub huge_page_tlb_size : usize , pub encoding_type : KeyEncodingType , pub full_scan_mode : bool , pub store_index_in_file : bool , }
    };
}

PlainTableFactoryOptions!()