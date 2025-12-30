// Generated macro for StartFfiInfo (struct)
macro_rules! Depcrate_shims_native_lib_trace_messagesStartFfiInfo {
() => {
// Module: crate::shims::native_lib::trace::messages
// Provides: {"StartFfiInfo"}
// Dependencies: {}
# [doc = " Information needed to begin tracing."] # [derive (serde :: Serialize , serde :: Deserialize , Debug , Clone)] pub struct StartFfiInfo { # [doc = " A vector of page addresses that store the miri heap which is accessible from C."] pub page_ptrs : Vec < usize > , # [doc = " The address of an allocation that can serve as a temporary stack."] # [doc = " This should be a leaked `Box<[u8; CALLBACK_STACK_SIZE]>` cast to an int."] pub stack_ptr : usize , }
};
}
