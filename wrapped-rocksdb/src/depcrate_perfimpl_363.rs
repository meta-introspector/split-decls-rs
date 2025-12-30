// Generated macro for impl_363 (impl)
macro_rules! Depcrate_perfimpl_363 {
() => {
// Module: crate::perf
// Provides: {"impl_363"}
// Dependencies: {}
impl PerfContext { # [doc = " Reset context"] pub fn reset (& mut self) { unsafe { ffi :: rocksdb_perfcontext_reset (self . inner) ; } } # [doc = " Get the report on perf"] pub fn report (& self , exclude_zero_counters : bool) -> String { unsafe { let ptr = ffi :: rocksdb_perfcontext_report (self . inner , c_uchar :: from (exclude_zero_counters)) ; let report = from_cstr (ptr) ; ffi :: rocksdb_free (ptr as * mut c_void) ; report } } # [doc = " Returns value of a metric"] pub fn metric (& self , id : PerfMetric) -> u64 { unsafe { ffi :: rocksdb_perfcontext_metric (self . inner , id as c_int) } } }
};
}
