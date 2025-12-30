// Generated macro for impl_209 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_209 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_209"}
// Dependencies: {}
impl History { # [doc = " Keep only the tag and creation"] fn forget (& self) -> Self { History { events : Vec :: new () , created : self . created , tag : self . tag } } # [doc = " Reconstruct the history relevant to `error_offset` by filtering"] # [doc = " only events whose range contains the offset we are interested in."] fn extract_relevant (& self , error_offset : u64 , error_kind : TransitionError) -> Self { History { events : self . events . iter () . filter (| e | e . transition_range . contains (& error_offset)) . filter (| e | e . transition . is_relevant (error_kind)) . cloned () . collect :: < Vec < _ > > () , created : self . created , tag : self . tag , } } }
};
}
