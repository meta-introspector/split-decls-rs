// Generated macro for impl_704 (impl)
macro_rules! Depcrate_vm_java_vmimpl_704 {
() => {
// Module: crate::vm::java_vm
// Provides: {"impl_704"}
// Dependencies: {}
impl Drop for AttachGuard < '_ > { fn drop (& mut self) { if let Err (err) = unsafe { self . detach_impl () } { log :: error ! ("Failed to detach current JNI thread: {err}") ; } } }
};
}
