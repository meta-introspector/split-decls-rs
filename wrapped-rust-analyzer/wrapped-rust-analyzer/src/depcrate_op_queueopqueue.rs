// Generated macro for OpQueue (struct)
macro_rules! Depcrate_op_queueOpQueue {
() => {
// Module: crate::op_queue
// Provides: {"OpQueue"}
// Dependencies: {}
# [doc = " A single-item queue that allows callers to request an operation to"] # [doc = " be performed later."] # [doc = ""] # [doc = " ```ignore"] # [doc = " let queue = OpQueue::default();"] # [doc = ""] # [doc = " // Request work to be done."] # [doc = " queue.request_op(\"user pushed a button\", ());"] # [doc = ""] # [doc = " // In a later iteration of the server loop, we start the work."] # [doc = " if let Some((_cause, ())) = queue.should_start_op() {"] # [doc = "     dbg!(\"Some slow operation here\");"] # [doc = " }"] # [doc = ""] # [doc = " // In an even later iteration of the server loop, we can see that the work"] # [doc = " // was completed."] # [doc = " if !queue.op_in_progress() {"] # [doc = "     dbg!(\"Work has been done!\");"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub (crate) struct OpQueue < Args = () , Output = () > { op_requested : Option < (Cause , Args) > , op_in_progress : bool , last_op_result : Option < Output > , }
};
}
