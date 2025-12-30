// Generated macro for CloseTriggerFrame (struct)
macro_rules! Depcrate_frameCloseTriggerFrame {
() => {
// Module: crate::frame
// Provides: {"CloseTriggerFrame"}
// Dependencies: {}
# [doc = " Instructs h3i to watch for certain incoming [`H3iFrame`]s. The incoming"] # [doc = " frames can either be supplied directly via [`CloseTriggerFrame::new`], or"] # [doc = " via a verification callback  passed to"] # [doc = " [`CloseTriggerFrame::new_with_comparator`]."] # [derive (Serialize , Clone)] pub struct CloseTriggerFrame { stream_id : u64 , comparator : Comparator , }
};
}
