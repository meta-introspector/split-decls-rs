// Generated macro for tests (module)
macro_rules! Depcrate_handletests {
() => {
// Module: crate::handle
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { Handle , HandleType } ; # [test] fn test_get_handle () { assert ! (Handle :: new (HandleType :: OutputHandle) . is_ok ()) ; assert ! (Handle :: new (HandleType :: InputHandle) . is_ok ()) ; assert ! (Handle :: new (HandleType :: CurrentOutputHandle) . is_ok ()) ; assert ! (Handle :: new (HandleType :: CurrentInputHandle) . is_ok ()) ; } }
};
}
