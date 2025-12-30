// Generated macro for Tracer (trait)
macro_rules! Depcrate_tree_builder_interfaceTracer {
() => {
// Module: crate::tree_builder::interface
// Provides: {"Tracer"}
// Dependencies: {}
# [doc = " Trace hooks for a garbage-collected DOM."] pub trait Tracer { type Handle ; # [doc = " Upon a call to `trace_handles`, the tree builder will call this method"] # [doc = " for each handle in its internal state."] fn trace_handle (& self , node : & Self :: Handle) ; }
};
}
