macro_rules! deps {
    () => {
        ReadOptions!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl Default for ReadOptions { fn default () -> Self { unsafe { Self { inner : ffi :: rocksdb_readoptions_create () , timestamp : None , iter_start_ts : None , iterate_upper_bound : None , iterate_lower_bound : None , } } } }
    };
}

impl_210!()