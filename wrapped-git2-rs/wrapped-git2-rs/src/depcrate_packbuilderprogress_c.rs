// Generated macro for progress_c (function)
macro_rules! Depcrate_packbuilderprogress_c {
() => {
// Module: crate::packbuilder
// Provides: {"progress_c"}
// Dependencies: {}
extern "C" fn progress_c (stage : raw :: git_packbuilder_stage_t , current : c_uint , total : c_uint , data : * mut c_void ,) -> c_int { unsafe { let stage = Binding :: from_raw (stage) ; let r = panic :: wrap (| | { let data = data as * mut Box < ProgressCb < '_ > > ; (* data) (stage , current , total) }) ; if r == Some (true) { 0 } else { - 1 } } }
};
}
