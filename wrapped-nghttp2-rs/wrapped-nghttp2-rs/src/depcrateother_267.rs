// Generated macro for other_267 (other)
macro_rules! Depcrateother_267 {
() => {
// Module: crate
// Provides: {"other_267"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns a pointer to a nghttp2_info struct with version information"] # [doc = " about the run-time library in use.  The |least_version| argument"] # [doc = " can be set to a 24 bit numerical value for the least accepted"] # [doc = " version number and if the condition is not met, this function will"] # [doc = " return a ``NULL``.  Pass in 0 to skip the version checking."] pub fn nghttp2_version (least_version : :: std :: os :: raw :: c_int) -> * mut nghttp2_info ; }
};
}
