// Generated macro for impl_282 (impl)
macro_rules! Depcrate_db_optionsimpl_282 {
() => {
// Module: crate::db_options
// Provides: {"impl_282"}
// Dependencies: {}
impl FifoCompactOptions { # [doc = " Sets the max table file size."] # [doc = ""] # [doc = " Once the total sum of table files reaches this, we will delete the oldest"] # [doc = " table file"] # [doc = ""] # [doc = " Default: 1GB"] pub fn set_max_table_files_size (& mut self , nbytes : u64) { unsafe { ffi :: rocksdb_fifo_compaction_options_set_max_table_files_size (self . inner , nbytes) ; } } }
};
}
