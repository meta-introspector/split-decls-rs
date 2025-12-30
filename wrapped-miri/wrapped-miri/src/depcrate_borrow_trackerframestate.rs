// Generated macro for FrameState (struct)
macro_rules! Depcrate_borrow_trackerFrameState {
() => {
// Module: crate::borrow_tracker
// Provides: {"FrameState"}
// Dependencies: {}
# [doc = " Per-call-stack-frame data for borrow tracking"] # [derive (Debug)] pub struct FrameState { # [doc = " If this frame is protecting any tags, they are listed here. We use this list to do"] # [doc = " incremental updates of the global list of protected tags stored in the"] # [doc = " `stacked_borrows::GlobalState` upon function return, and if we attempt to pop a protected"] # [doc = " tag, to identify which call is responsible for protecting the tag."] # [doc = " See `Stack::item_invalidated` for more explanation."] # [doc = " Tree Borrows also needs to know which allocation these tags"] # [doc = " belong to so that it can perform a read through them immediately before"] # [doc = " the frame gets popped."] # [doc = ""] # [doc = " This will contain one tag per reference passed to the function, so"] # [doc = " a size of 2 is enough for the vast majority of functions."] protected_tags : SmallVec < [(AllocId , BorTag) ; 2] > , }
};
}
