// Generated macro for JAVA_VM_SINGLETON (static)
macro_rules! Depcrate_vm_java_vmJAVA_VM_SINGLETON {
() => {
// Module: crate::vm::java_vm
// Provides: {"JAVA_VM_SINGLETON"}
// Dependencies: {}
# [doc = " The `jni-rs` crate makes the assumption that it's not possible to create more than one Java VM"] # [doc = " per-process, or even re-initialize a JavaVM that is \"destroyed\"."] # [doc = ""] # [doc = " This allows us to save a global pointer for the JavaVM."] # [doc = ""] # [doc = " We also guarantee that if you currently have an [`AttachGuard`] thread attachment (or a `Env`"] # [doc = " reference), that implies that [`JavaVM::singleton()`] has been initialized and will return a"] # [doc = " valid [`JavaVM`]."] # [doc = ""] # [doc = " For example, this guarantee is relied on internally to avoid redundantly saving JavaVM pointers"] # [doc = " if know we can assume that `JavaVM::singleton()` will return a `JavaVM` when needed."] static JAVA_VM_SINGLETON : once_cell :: sync :: OnceCell < JavaVM > = once_cell :: sync :: OnceCell :: new () ;
};
}
