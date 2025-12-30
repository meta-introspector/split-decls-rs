// Generated macro for ScopeToken (struct)
macro_rules! Depcrate_vm_java_vmScopeToken {
() => {
// Module: crate::vm::java_vm
// Provides: {"ScopeToken"}
// Dependencies: {}
# [doc = " A non-Send, non-Sync token, with no `const` constructor, representing a"] # [doc = " local scope when attaching the current thread to a Java VM."] # [doc = ""] # [doc = " This gives us something for an [`AttachGuard`] to borrow that's not likely"] # [doc = " to to be accidentally made `'static`."] # [doc = ""] # [doc = " This is only relevant for `unsafe` code that is manually creating"] # [doc = " [`AttachGuard`]s."] # [doc = ""] # [doc = " See: [JavaVM::attach_current_thread_guard] and [JavaVM::get_env_attachment]."] # [derive (Debug , Default)] pub struct ScopeToken { _non_send_sync : std :: marker :: PhantomData < * const () > , }
};
}
