// Generated macro for TestGroup (struct)
macro_rules! Depcrate_test_dashboardTestGroup {
() => {
// Module: crate::test_dashboard
// Provides: {"TestGroup"}
// Dependencies: {}
# [derive (Template)] # [template (path = "test_group.askama")] # [doc = " Represents a group of tests"] struct TestGroup < 'a > { name : String , # [doc = " Tests located directly in this directory"] root_tests : Vec < (String , Test < 'a >) > , # [doc = " Nested directories with additional tests"] groups : Vec < (String , TestGroup < 'a >) > , }
};
}
