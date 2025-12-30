// Generated macro for impl_203 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_203 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_203"}
// Dependencies: {}
impl HistoryData { fn extend (& mut self , new_history : History , tag_name : & 'static str , show_initial_state : bool) { let History { tag , created , events } = new_history ; let this = format ! ("the {tag_name} tag {tag:?}") ; let msg_initial_state = format ! (", in the initial state {}" , created . 1) ; let msg_creation = format ! ("{this} was created here{maybe_msg_initial_state}" , maybe_msg_initial_state = if show_initial_state { & msg_initial_state } else { "" } ,) ; self . events . push ((Some (created . 0 . data ()) , msg_creation)) ; for & Event { transition , is_foreign , access_cause , access_range , span , transition_range : _ , } in & events { let access = access_cause . print_as_access (is_foreign) ; let access_range_text = match access_range { Some (r) => format ! ("at offsets {r:?}") , None => format ! ("on every location previously accessed by this tag") , } ; self . events . push ((Some (span . data ()) , format ! ("{this} later transitioned to {endpoint} due to a {access} {access_range_text}" , endpoint = transition . endpoint ()) ,)) ; self . events . push ((None , format ! ("this transition corresponds to {}" , transition . summary ()))) ; } } }
};
}
