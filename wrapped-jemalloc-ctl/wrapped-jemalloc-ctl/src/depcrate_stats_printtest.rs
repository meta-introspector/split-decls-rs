// Generated macro for test (module)
macro_rules! Depcrate_stats_printtest {
() => {
// Module: crate::stats_print
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn basic () { let mut buf = vec ! [] ; stats_print (& mut buf , Options :: default ()) . unwrap () ; println ! ("{}" , String :: from_utf8 (buf) . unwrap ()) ; } # [test] fn all_options () { let mut buf = vec ! [] ; let options = Options { json_format : true , skip_constants : true , skip_merged_arenas : true , skip_per_arena : true , skip_bin_size_classes : true , skip_large_size_classes : true , skip_mutex_statistics : true , _p : () , } ; stats_print (& mut buf , options) . unwrap () ; println ! ("{}" , String :: from_utf8 (buf) . unwrap ()) ; } }
};
}
