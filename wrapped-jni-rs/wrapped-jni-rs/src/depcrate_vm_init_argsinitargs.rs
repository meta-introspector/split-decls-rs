// Generated macro for InitArgs (struct)
macro_rules! Depcrate_vm_init_argsInitArgs {
() => {
// Module: crate::vm::init_args
// Provides: {"InitArgs"}
// Dependencies: {}
# [doc = " JavaVM InitArgs."] # [doc = ""] # [doc = " *This API requires \"invocation\" feature to be enabled,"] # [doc = " see [\"Launching JVM from Rust\"](struct.JavaVM.html#launching-jvm-from-rust).*"] # [derive (Debug)] pub struct InitArgs < 'a > { inner : JavaVMInitArgs , _opts : Vec < JavaVMOption > , _opt_strings : Vec < Cow < 'a , CStr > > , }
};
}
