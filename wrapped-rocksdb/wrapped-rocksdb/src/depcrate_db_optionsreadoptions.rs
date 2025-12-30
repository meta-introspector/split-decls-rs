// Generated macro for ReadOptions (struct)
macro_rules! Depcrate_db_optionsReadOptions {
() => {
// Module: crate::db_options
// Provides: {"ReadOptions"}
// Dependencies: {}
pub struct ReadOptions { pub (crate) inner : * mut ffi :: rocksdb_readoptions_t , timestamp : Option < Vec < u8 > > , iter_start_ts : Option < Vec < u8 > > , iterate_upper_bound : Option < Vec < u8 > > , iterate_lower_bound : Option < Vec < u8 > > , }
};
}
