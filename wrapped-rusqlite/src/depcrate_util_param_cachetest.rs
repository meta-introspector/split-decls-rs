// Generated macro for test (module)
macro_rules! Depcrate_util_param_cachetest {
() => {
// Module: crate::util::param_cache
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_cache () { let p = ParamIndexCache :: default () ; let v = p . get_or_insert_with ("foo" , | cstr | { assert_eq ! (cstr . to_str () . unwrap () , "foo") ; Some (3) }) ; assert_eq ! (v , Some (3)) ; let v = p . get_or_insert_with ("foo" , | _ | { panic ! ("shouldn't be called this time") ; }) ; assert_eq ! (v , Some (3)) ; let v = p . get_or_insert_with ("gar\0bage" , | _ | { panic ! ("shouldn't be called here either") ; }) ; assert_eq ! (v , None) ; let v = p . get_or_insert_with ("bar" , | cstr | { assert_eq ! (cstr . to_str () . unwrap () , "bar") ; None }) ; assert_eq ! (v , None) ; let v = p . get_or_insert_with ("bar" , | cstr | { assert_eq ! (cstr . to_str () . unwrap () , "bar") ; Some (30) }) ; assert_eq ! (v , Some (30)) ; } }
};
}
