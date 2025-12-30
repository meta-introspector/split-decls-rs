// Generated macro for test (module)
macro_rules! Depcrate_vm_java_vmtest {
() => {
// Module: crate::vm::java_vm
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { AttachGuard , ScopeToken } ; static_assertions :: assert_not_impl_any ! (ScopeToken : Send) ; static_assertions :: assert_not_impl_any ! (ScopeToken : Sync) ; static_assertions :: assert_not_impl_any ! (AttachGuard : Send) ; static_assertions :: assert_not_impl_any ! (AttachGuard : Sync) ; }
};
}
