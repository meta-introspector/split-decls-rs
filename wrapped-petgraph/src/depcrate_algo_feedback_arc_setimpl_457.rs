// Generated macro for impl_457 (impl)
macro_rules! Depcrate_algo_feedback_arc_setimpl_457 {
() => {
// Module: crate::algo::feedback_arc_set
// Provides: {"impl_457"}
// Dependencies: {}
impl Index < FasNodeIndex > for FasNodeContainer { type Output = LinkedListEntry < FasNode , FasNodeIndex > ; fn index (& self , index : FasNodeIndex) -> & Self :: Output { & self . nodes [index . 0] } }
};
}
