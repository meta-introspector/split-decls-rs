// Generated macro for TLSAttachGuard (struct)
macro_rules! Depcrate_vm_java_vmTLSAttachGuard {
() => {
// Module: crate::vm::java_vm
// Provides: {"TLSAttachGuard"}
// Dependencies: {}
# [derive (Debug)] struct TLSAttachGuard { env : * mut jni_sys :: JNIEnv , # [doc = " A call std::thread::current() function can panic in case the local data has been destroyed"] # [doc = " before the thead local variables. The possibility of this happening depends on the platform"] # [doc = " implementation of the sys_common::thread_local_dtor::register_dtor_fallback."] # [doc = ""] # [doc = " Since this struct will be saved as a thread-local variable, we capture the thread meta-data"] # [doc = " during creation"] thread : Thread , }
};
}
