macro_rules! deps {
    () => {
        FifoCompactOptions!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl FifoCompactOptions { # [doc = " Sets the max table file size."] # [doc = ""] # [doc = " Once the total sum of table files reaches this, we will delete the oldest"] # [doc = " table file"] # [doc = ""] # [doc = " Default: 1GB"] pub fn set_max_table_files_size (& mut self , nbytes : u64) { unsafe { ffi :: rocksdb_fifo_compaction_options_set_max_table_files_size (self . inner , nbytes) ; } } }
    };
}

impl_226!()