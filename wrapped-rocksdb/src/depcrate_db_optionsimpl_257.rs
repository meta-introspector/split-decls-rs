// Generated macro for impl_257 (impl)
macro_rules! Depcrate_db_optionsimpl_257 {
() => {
// Module: crate::db_options
// Provides: {"impl_257"}
// Dependencies: {}
impl FlushOptions { pub fn new () -> FlushOptions { FlushOptions :: default () } # [doc = " Waits until the flush is done."] # [doc = ""] # [doc = " Default: true"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::FlushOptions;"] # [doc = ""] # [doc = " let mut options = FlushOptions::default();"] # [doc = " options.set_wait(false);"] # [doc = " ```"] pub fn set_wait (& mut self , wait : bool) { unsafe { ffi :: rocksdb_flushoptions_set_wait (self . inner , c_uchar :: from (wait)) ; } } }
};
}
