// Generated macro for impl_163 (impl)
macro_rules! Depcrate_op_queueimpl_163 {
() => {
// Module: crate::op_queue
// Provides: {"impl_163"}
// Dependencies: {}
impl < Args : std :: fmt :: Debug , Output > OpQueue < Args , Output > { # [doc = " Request an operation to start."] pub (crate) fn request_op (& mut self , reason : Cause , args : Args) { self . op_requested = Some ((reason , args)) ; } # [doc = " If there was an operation requested, mark this queue as"] # [doc = " started and return the request arguments."] pub (crate) fn should_start_op (& mut self) -> Option < (Cause , Args) > { if self . op_in_progress { return None ; } self . op_in_progress = self . op_requested . is_some () ; self . op_requested . take () } # [doc = " Mark an operation as completed."] pub (crate) fn op_completed (& mut self , result : Output) { assert ! (self . op_in_progress) ; self . op_in_progress = false ; self . last_op_result = Some (result) ; } # [doc = " Get the result of the last operation."] pub (crate) fn last_op_result (& self) -> Option < & Output > { self . last_op_result . as_ref () } pub (crate) fn op_in_progress (& self) -> bool { self . op_in_progress } pub (crate) fn op_requested (& self) -> bool { self . op_requested . is_some () } }
};
}
