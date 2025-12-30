// Generated macro for impl_174 (impl)
macro_rules! Depcrate_queueimpl_174 {
() => {
// Module: crate::queue
// Provides: {"impl_174"}
// Dependencies: {}
impl GlobalQueueIdentifier { # [doc = " Convert and consume [GlobalQueueIdentifier] into its raw value."] pub fn to_identifier (self) -> isize { match self { GlobalQueueIdentifier :: Priority (queue_priority) => queue_priority . 0 as isize , GlobalQueueIdentifier :: QualityOfService (qos_class) => qos_class . 0 as isize , } } }
};
}
