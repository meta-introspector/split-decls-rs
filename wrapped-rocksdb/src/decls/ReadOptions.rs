macro_rules! ReadOptions {
    () => {
        pub struct ReadOptions { pub (crate) inner : * mut ffi :: rocksdb_readoptions_t , timestamp : Option < Vec < u8 > > , iter_start_ts : Option < Vec < u8 > > , iterate_upper_bound : Option < Vec < u8 > > , iterate_lower_bound : Option < Vec < u8 > > , }
    };
}

ReadOptions!()