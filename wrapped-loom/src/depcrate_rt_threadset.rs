// Generated macro for Set (struct)
macro_rules! Depcrate_rt_threadSet {
() => {
// Module: crate::rt::thread
// Provides: {"Set"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Set { # [doc = " Unique execution identifier"] execution_id : execution :: Id , # [doc = " Set of threads"] threads : Vec < Thread > , # [doc = " Currently scheduled thread."] # [doc = ""] # [doc = " `None` signifies that no thread is runnable."] active : Option < usize > , # [doc = " Sequential consistency causality. All sequentially consistent operations"] # [doc = " synchronize with this causality."] pub seq_cst_causality : VersionVec , # [doc = " `tracing` span used as the parent for new thread spans."] iteration_span : tracing :: Span , }
};
}
