// Generated macro for impl_707 (impl)
macro_rules! Depcrate_vm_java_vmimpl_707 {
() => {
// Module: crate::vm::java_vm
// Provides: {"impl_707"}
// Dependencies: {}
impl TLSAttachGuard { # [doc = " Detach a thread before the thread terminates **IFF** it was previously attached via"] # [doc = " [`JavaVM::attach_current_thread`] **AND** there is no active [`AttachGuard`] in use"] # [doc = " for this thread."] fn detach () -> Result < () > { if THREAD_GUARD_NEST_LEVEL . get () != 0 { return Err (Error :: ThreadAttachmentGuarded) ; } THREAD_ATTACH_GUARD . with (move | f | { if let Some (guard) = f . borrow_mut () . take () { let res = unsafe { guard . detach_impl () } ; std :: mem :: forget (guard) ; res } else { Ok (()) } }) } unsafe fn attach_current_thread < 'local > (java_vm : & JavaVM , config : & AttachConfig ,) -> Result < AttachGuard < 'local > > { let thread = current () ; let env = sys_attach_current_thread (java_vm , config , & thread) ? ; THREAD_ATTACH_GUARD . with (move | f | { * f . borrow_mut () = Some (Self { env , thread : current () , }) ; }) ; Ok (unsafe { AttachGuard :: from_unowned (env) }) } # [doc = " Detach the current thread after checking there are no active [`AttachGuard`]s"] # [doc = ""] # [doc = " # Safety"] # [doc = " Since this is used in the implementation of `Drop` you must make sure"] # [doc = " to not let `Drop` run if this is called explicitly."] unsafe fn detach_impl (& self) -> Result < () > { sys_detach_current_thread (self . env , & self . thread) } }
};
}
