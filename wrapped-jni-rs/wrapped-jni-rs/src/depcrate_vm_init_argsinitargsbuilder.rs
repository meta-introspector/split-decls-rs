// Generated macro for InitArgsBuilder (struct)
macro_rules! Depcrate_vm_init_argsInitArgsBuilder {
() => {
// Module: crate::vm::init_args
// Provides: {"InitArgsBuilder"}
// Dependencies: {}
# [doc = " Builder for JavaVM InitArgs."] # [doc = ""] # [doc = " *This API requires \"invocation\" feature to be enabled,"] # [doc = " see [\"Launching JVM from Rust\"](struct.JavaVM.html#launching-jvm-from-rust).*"] # [derive (Debug)] pub struct InitArgsBuilder < 'a > { opts : Result < Vec < Cow < 'a , CStr > > , JvmError > , ignore_unrecognized : bool , version : JNIVersion , }
};
}
