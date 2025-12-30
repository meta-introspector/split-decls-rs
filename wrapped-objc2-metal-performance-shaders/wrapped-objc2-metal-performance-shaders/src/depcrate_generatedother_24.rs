// Generated macro for other_24 (other)
macro_rules! Depcrate_generatedother_24 {
() => {
// Module: crate::generated
// Provides: {"other_24"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Set the timeout after which unused cached MTLHeaps are released"] # [doc = ""] # [doc = " MPS maintains a private set of MTLHeaps attached to each MTLCommandBuffer"] # [doc = " for use by temporary images, matrices, vectors and states, and also for its own"] # [doc = " private usage for temporary storage in some (typically multipass) filters. When the"] # [doc = " command buffer completes, these are returned to a MTLDevice level cache for reuse."] # [doc = " If it is not reused within the heap cache duration, then the MTLHeaps are released"] # [doc = " and the memory is returned to the operating system for general reuse. The intent"] # [doc = " of this second level cache is to avoid surrendering the GPU performance advantage"] # [doc = " on repetitive workloads to  allocation, zero-fill and deallocation and reallocation"] # [doc = " of large MTLHeaps, which otherwise can easily occur."] # [doc = ""] # [doc = " Default: 5s."] # [doc = ""] # [doc = ""] # [doc = " Parameter `cmdBuf`: The scope over which to set the heap cache duration. If the MTLCommandBuffer"] # [doc = " has already been committed, behavior is undefined."] # [doc = ""] # [doc = " Parameter `seconds`: The number of seconds to cache used MTLHeaps before retiring them."] # [doc = " NaN will be interpeted as 0."] pub fn MPSSetHeapCacheDuration (cmd_buf : & ProtocolObject < dyn MTLCommandBuffer > , seconds : c_double ,) ; }
};
}
