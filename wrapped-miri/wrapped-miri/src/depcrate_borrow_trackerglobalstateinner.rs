// Generated macro for GlobalStateInner (struct)
macro_rules! Depcrate_borrow_trackerGlobalStateInner {
() => {
// Module: crate::borrow_tracker
// Provides: {"GlobalStateInner"}
// Dependencies: {}
# [doc = " Extra global state, available to the memory access hooks."] # [derive (Debug)] pub struct GlobalStateInner { # [doc = " Borrow tracker method currently in use."] borrow_tracker_method : BorrowTrackerMethod , # [doc = " Next unused pointer ID (tag)."] next_ptr_tag : BorTag , # [doc = " Table storing the \"root\" tag for each allocation."] # [doc = " The root tag is the one used for the initial pointer."] # [doc = " We need this in a separate table to handle cyclic statics."] root_ptr_tags : FxHashMap < AllocId , BorTag > , # [doc = " All currently protected tags."] # [doc = " We add tags to this when they are created with a protector in `reborrow`, and"] # [doc = " we remove tags from this when the call which is protecting them returns, in"] # [doc = " `GlobalStateInner::end_call`. See `Stack::item_invalidated` for more details."] protected_tags : FxHashMap < BorTag , ProtectorKind > , # [doc = " The pointer ids to trace"] tracked_pointer_tags : FxHashSet < BorTag > , # [doc = " Whether to recurse into datatypes when searching for pointers to retag."] retag_fields : RetagFields , }
};
}
