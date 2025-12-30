// Generated macro for macro_705 (macro)
macro_rules! Depcrate_vm_java_vmmacro_705 {
() => {
// Module: crate::vm::java_vm
// Provides: {"macro_705"}
// Dependencies: {}
thread_local ! { # [cfg_attr (target_os = "android" , allow (clippy :: missing_const_for_thread_local))] static THREAD_ATTACH_GUARD : RefCell < Option < TLSAttachGuard >> = const { RefCell :: new (None) } }
};
}
