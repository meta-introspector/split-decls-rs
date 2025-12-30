// Generated macro for dispatch_io_handler_t (type)
macro_rules! Depcrate_generateddispatch_io_handler_t {
() => {
// Module: crate::generated
// Provides: {"dispatch_io_handler_t"}
// Dependencies: {}
# [doc = " The prototype of I/O handler blocks for dispatch I/O operations."] # [doc = ""] # [doc = ""] # [doc = " Parameter `done`: A flag indicating whether the operation is complete."] # [doc = ""] # [doc = " Parameter `data`: The data object to be handled."] # [doc = ""] # [doc = " Parameter `error`: An errno condition for the operation."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/dispatch/dispatch_io_handler_t?language=objc)"] # [cfg (feature = "block2")] pub type dispatch_io_handler_t = * mut block2 :: DynBlock < dyn Fn (bool , * mut DispatchData , c_int) > ;
};
}
