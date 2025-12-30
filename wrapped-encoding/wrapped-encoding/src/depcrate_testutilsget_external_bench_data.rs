// Generated macro for get_external_bench_data (function)
macro_rules! Depcrate_testutilsget_external_bench_data {
() => {
// Module: crate::testutils
// Provides: {"get_external_bench_data"}
// Dependencies: {}
# [doc = " Returns a longer text used for external data benchmarks."] # [doc = " This can be overriden with an environment variable `EXTERNAL_BENCH_DATA`,"] # [doc = " or it will use a built-in sample data (of about 100KB)."] pub fn get_external_bench_data () -> Vec < u8 > { use std :: env ; use std :: io :: Read ; use std :: fs :: File ; use std :: path :: Path ; static LONGER_TEXT : & 'static [u8] = include_bytes ! ("examples/outer-space-treaty.html") ; match env :: var ("EXTERNAL_BENCH_DATA") { Ok (path) => { let path = Path :: new (& path) ; let mut file = File :: open (& path) . ok () . expect ("cannot read an external bench data") ; let mut ret = Vec :: new () ; file . read_to_end (& mut ret) . ok () . expect ("cannot read an external bench data") ; ret } Err (..) => { LONGER_TEXT . to_vec () } } }
};
}
