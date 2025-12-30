// Generated macro for impl_254 (impl)
macro_rules! Depcrate_frameimpl_254 {
() => {
// Module: crate::frame
// Provides: {"impl_254"}
// Dependencies: {}
impl PartialEq for CloseTriggerFrame { fn eq (& self , other : & Self) -> bool { match (& self . comparator , & other . comparator) { (Comparator :: Frame (this_frame) , Comparator :: Frame (other_frame)) => self . stream_id == other . stream_id && this_frame == other_frame , _ => false , } } }
};
}
