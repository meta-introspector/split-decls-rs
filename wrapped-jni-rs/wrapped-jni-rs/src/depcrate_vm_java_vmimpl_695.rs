// Generated macro for impl_695 (impl)
macro_rules! Depcrate_vm_java_vmimpl_695 {
() => {
// Module: crate::vm::java_vm
// Provides: {"impl_695"}
// Dependencies: {}
impl < 'a > AttachConfig < 'a > { # [doc = " Creates a new `AttachConfig` with default values."] pub fn new () -> Self { Self :: default () } # [doc = " Sets whether the attachment should be owned by the current scope, such"] # [doc = " that the thread will be automatically detached when the attachment guard"] # [doc = " is dropped."] # [doc = ""] # [doc = " The default is `false`, so the thread will be attached permanently."] # [doc = ""] # [doc = " It is normally best to attach permanently because it can reduce the cost"] # [doc = " of repeatedly attaching and detaching threads."] pub fn scoped (mut self , scoped : bool) -> Self { self . scoped = scoped ; self } # [doc = " Sets the name of the thread as seen by the JVM and operating system."] pub fn name < S : AsRef < str > > (mut self , name : S) -> Self { self . name = Some (JNIString :: from (name . as_ref ())) ; self } # [doc = " Specifies a global reference to a `ThreadGroup` that the thread should"] # [doc = " be associated with."] pub fn group (mut self , group : & 'a Global < JObject < 'static > >) -> Self { self . group = Some (group) ; self } }
};
}
