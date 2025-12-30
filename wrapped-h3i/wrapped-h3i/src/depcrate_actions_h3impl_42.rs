// Generated macro for impl_42 (impl)
macro_rules! Depcrate_actions_h3impl_42 {
() => {
// Module: crate::actions::h3
// Provides: {"impl_42"}
// Dependencies: {}
impl WaitingFor { pub (crate) fn is_empty (& self) -> bool { self . 0 . values () . all (| v | v . is_empty ()) } pub (crate) fn add_wait (& mut self , stream_event : & StreamEvent) { self . 0 . entry (stream_event . stream_id) . or_default () . push (* stream_event) ; } pub (crate) fn remove_wait (& mut self , stream_event : StreamEvent) { if let Some (waits) = self . 0 . get_mut (& stream_event . stream_id) { let old_len = waits . len () ; waits . retain (| wait | wait != & stream_event) ; let new_len = waits . len () ; if old_len != new_len { log :: info ! ("No longer waiting for {stream_event:?}") ; } } } pub (crate) fn clear_waits_on_stream (& mut self , stream_id : u64) { if let Some (waits) = self . 0 . get_mut (& stream_id) { if ! waits . is_empty () { log :: info ! ("Clearing all waits for stream {stream_id}") ; waits . clear () ; } } } }
};
}
