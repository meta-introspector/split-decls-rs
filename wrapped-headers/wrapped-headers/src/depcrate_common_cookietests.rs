// Generated macro for tests (module)
macro_rules! Depcrate_common_cookietests {
() => {
// Module: crate::common::cookie
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: Cookie ; # [test] fn test_parse () { let cookie = test_decode :: < Cookie > (& ["foo=bar"]) . unwrap () ; assert_eq ! (cookie . get ("foo") , Some ("bar")) ; assert_eq ! (cookie . get ("bar") , None) ; } # [test] fn test_multipe_same_name () { let cookie = test_decode :: < Cookie > (& ["foo=bar; foo=baz"]) . unwrap () ; assert_eq ! (cookie . get ("foo") , Some ("bar")) ; } # [test] fn test_multipe_lines () { let cookie = test_decode :: < Cookie > (& ["foo=bar" , "lol = cat"]) . unwrap () ; assert_eq ! (cookie . get ("foo") , Some ("bar")) ; assert_eq ! (cookie . get ("lol") , Some ("cat")) ; } }
};
}
