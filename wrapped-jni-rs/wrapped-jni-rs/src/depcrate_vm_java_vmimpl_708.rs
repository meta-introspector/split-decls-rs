// Generated macro for impl_708 (impl)
macro_rules! Depcrate_vm_java_vmimpl_708 {
() => {
// Module: crate::vm::java_vm
// Provides: {"impl_708"}
// Dependencies: {}
impl Drop for TLSAttachGuard { fn drop (& mut self) { if let Err (e) = unsafe { self . detach_impl () } { error ! ("Error detaching current thread: {:#?}\nThread {} id={:?}" , e , self . thread . name () . unwrap_or_default () , self . thread . id () ,) ; } } }
};
}
