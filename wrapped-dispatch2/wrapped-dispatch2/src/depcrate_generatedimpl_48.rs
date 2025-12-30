// Generated macro for impl_48 (impl)
macro_rules! Depcrate_generatedimpl_48 {
() => {
// Module: crate::generated
// Provides: {"impl_48"}
// Dependencies: {}
impl DispatchQueue { # [doc = " Returns the subsystem-specific context associated with a dispatch queue, for"] # [doc = " a key unique to the subsystem."] # [doc = ""] # [doc = ""] # [doc = " Returns the context for the specified key if it has been set on the specified"] # [doc = " queue."] # [doc = ""] # [doc = ""] # [doc = " Parameter `queue`: The dispatch queue to query."] # [doc = " The result of passing NULL in this parameter is undefined."] # [doc = ""] # [doc = ""] # [doc = " Parameter `key`: The key to get the context for, typically a pointer to a static variable"] # [doc = " specific to the subsystem. Keys are only compared as pointers and never"] # [doc = " dereferenced. Passing a string constant directly is not recommended."] # [doc = ""] # [doc = ""] # [doc = " Returns: The context for the specified key or NULL if no context was found."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `queue` possibly has additional threading requirements."] # [doc = " - `key` must be a valid pointer."] # [doc (alias = "dispatch_queue_get_specific")] # [must_use] # [inline] pub unsafe fn specific (& self , key : NonNull < c_void >) -> * mut c_void { extern "C" { fn dispatch_queue_get_specific (queue : & DispatchQueue , key : NonNull < c_void > ,) -> * mut c_void ; } unsafe { dispatch_queue_get_specific (self , key) } } }
};
}
