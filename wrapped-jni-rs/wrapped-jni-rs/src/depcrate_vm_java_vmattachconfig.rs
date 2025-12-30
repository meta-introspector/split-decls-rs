// Generated macro for AttachConfig (struct)
macro_rules! Depcrate_vm_java_vmAttachConfig {
() => {
// Module: crate::vm::java_vm
// Provides: {"AttachConfig"}
// Dependencies: {}
# [doc = " Configuration options for attaching the current thread to a Java VM."] # [derive (Debug , Default)] pub struct AttachConfig < 'a > { scoped : bool , name : Option < JNIString > , group : Option < & 'a Global < JObject < 'static > > > , }
};
}
