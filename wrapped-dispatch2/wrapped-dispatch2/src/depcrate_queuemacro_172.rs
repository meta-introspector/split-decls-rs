// Generated macro for macro_172 (macro)
macro_rules! Depcrate_queuemacro_172 {
() => {
// Module: crate::queue
// Provides: {"macro_172"}
// Dependencies: {}
enum_with_val ! { # [doc = " Queue priority."] # [doc (alias = "dispatch_queue_priority_t")] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct DispatchQueueGlobalPriority (pub c_long) { # [doc = " High priority."] # [doc (alias = "DISPATCH_QUEUE_PRIORITY_HIGH")] High = 0x2 , # [doc = " Default priority."] # [doc (alias = "DISPATCH_QUEUE_PRIORITY_DEFAULT")] Default = 0x0 , # [doc = " Low priority."] # [doc (alias = "DISPATCH_QUEUE_PRIORITY_LOW")] Low = - 0x2 , # [doc = " Background priority."] # [doc (alias = "DISPATCH_QUEUE_PRIORITY_BACKGROUND")] Background = u16 :: MIN as c_long , } }
};
}
