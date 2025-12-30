// Generated macro for macro_699 (macro)
macro_rules! Depcrate_vm_java_vmmacro_699 {
() => {
// Module: crate::vm::java_vm
// Provides: {"macro_699"}
// Dependencies: {}
thread_local ! { # [cfg_attr (target_os = "android" , allow (clippy :: missing_const_for_thread_local))] static THREAD_ATTACHMENT : Cell <* mut jni_sys :: JNIEnv > = const { Cell :: new (std :: ptr :: null_mut ()) } ; # [cfg_attr (target_os = "android" , allow (clippy :: missing_const_for_thread_local))] static THREAD_GUARD_NEST_LEVEL : Cell < usize > = const { Cell :: new (0) } ; }
};
}
