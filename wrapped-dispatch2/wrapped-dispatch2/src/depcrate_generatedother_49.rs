// Generated macro for other_49 (other)
macro_rules! Depcrate_generatedother_49 {
() => {
// Module: crate::generated
// Provides: {"other_49"}
// Dependencies: {}
extern "C" { # [doc = " Returns the current subsystem-specific context for a key unique to the"] # [doc = " subsystem."] # [doc = ""] # [doc = ""] # [doc = " When called from a block executing on a queue, returns the context for the"] # [doc = " specified key if it has been set on the queue, otherwise returns the result"] # [doc = " of dispatch_get_specific() executed on the queue's target queue or NULL"] # [doc = " if the current queue is a global concurrent queue."] # [doc = ""] # [doc = ""] # [doc = " Parameter `key`: The key to get the context for, typically a pointer to a static variable"] # [doc = " specific to the subsystem. Keys are only compared as pointers and never"] # [doc = " dereferenced. Passing a string constant directly is not recommended."] # [doc = ""] # [doc = ""] # [doc = " Returns: The context for the specified key or NULL if no context was found."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `key` must be a valid pointer."] # [must_use] pub fn dispatch_get_specific (key : NonNull < c_void >) -> * mut c_void ; }
};
}
