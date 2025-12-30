// Generated macro for TestCase (struct)
macro_rules! Depcrate_testTestCase {
() => {
// Module: crate::test
// Provides: {"TestCase"}
// Dependencies: {}
# [doc = " A test case. A test case consists of a set of named attributes. Every"] # [doc = " attribute in the test case must be consumed exactly once; this helps catch"] # [doc = " typos and omissions."] # [doc = ""] # [doc = " Requires the `alloc` default feature to be enabled."] # [derive (Debug)] # [allow (clippy :: module_name_repetitions)] pub struct TestCase { attributes : Vec < (String , String , bool) > , }
};
}
