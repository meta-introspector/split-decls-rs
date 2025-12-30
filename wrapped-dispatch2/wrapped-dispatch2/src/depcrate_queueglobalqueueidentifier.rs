// Generated macro for GlobalQueueIdentifier (enum)
macro_rules! Depcrate_queueGlobalQueueIdentifier {
() => {
// Module: crate::queue
// Provides: {"GlobalQueueIdentifier"}
// Dependencies: {}
# [doc = " Global queue identifier definition for [`DispatchQueue::new`] and [`DispatchQueue::new_with_target`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub enum GlobalQueueIdentifier { # [doc = " Standard priority based queue."] Priority (DispatchQueueGlobalPriority) , # [doc = " Quality of service priority based queue."] QualityOfService (DispatchQoS) , }
};
}
