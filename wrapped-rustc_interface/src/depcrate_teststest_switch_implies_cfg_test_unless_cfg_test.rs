// Generated macro for test_switch_implies_cfg_test_unless_cfg_test (function)
macro_rules! Depcrate_teststest_switch_implies_cfg_test_unless_cfg_test {
() => {
// Module: crate::tests
// Provides: {"test_switch_implies_cfg_test_unless_cfg_test"}
// Dependencies: {}
# [test] fn test_switch_implies_cfg_test_unless_cfg_test () { sess_and_cfg (& ["--test" , "--cfg=test"] , | _sess , cfg | { let mut test_items = cfg . iter () . filter (| & & (name , _) | name == sym :: test) ; assert ! (test_items . next () . is_some ()) ; assert ! (test_items . next () . is_none ()) ; }) ; }
};
}
