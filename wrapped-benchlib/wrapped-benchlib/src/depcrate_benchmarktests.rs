// Generated macro for tests (module)
macro_rules! Depcrate_benchmarktests {
() => {
// Module: crate::benchmark
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: benchmark :: passes_filter ; # [test] fn test_passes_filter_no_filter () { assert ! (passes_filter ("foo" , & [] , & [])) ; } # [test] fn test_passes_filter_include () { assert ! (! passes_filter ("foo" , & [] , & ["bar" . to_string ()])) ; assert ! (! passes_filter ("foo" , & [] , & ["foobar" . to_string ()])) ; assert ! (passes_filter ("foo" , & [] , & ["f" . to_string ()])) ; assert ! (passes_filter ("foo" , & [] , & ["foo" . to_string ()])) ; assert ! (passes_filter ("foo" , & [] , & ["bar" . to_string () , "baz" . to_string () , "foo" . to_string ()])) ; } # [test] fn test_passes_filter_exclude () { assert ! (passes_filter ("foo" , & ["bar" . to_string ()] , & [])) ; assert ! (passes_filter ("foo" , & ["foobar" . to_string ()] , & [])) ; assert ! (! passes_filter ("foo" , & ["f" . to_string ()] , & [])) ; assert ! (! passes_filter ("foo" , & ["foo" . to_string ()] , & [])) ; assert ! (! passes_filter ("foo" , & ["bar" . to_string () , "baz" . to_string () , "foo" . to_string ()] , & [])) ; } # [test] fn test_passes_filter_include_exclude () { assert ! (! passes_filter ("foo" , & ["bar" . to_string ()] , & ["baz" . to_string ()])) ; assert ! (passes_filter ("foo" , & ["bar" . to_string ()] , & ["foo" . to_string ()])) ; assert ! (! passes_filter ("foo" , & ["foo" . to_string ()] , & ["bar" . to_string ()])) ; assert ! (! passes_filter ("foo" , & ["foo" . to_string ()] , & ["foo" . to_string ()])) ; } }
};
}
