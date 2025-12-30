// Generated macro for callback_impl (function)
macro_rules! Depcrate_fseventcallback_impl {
() => {
// Module: crate::fsevent
// Provides: {"callback_impl"}
// Dependencies: {}
unsafe fn callback_impl (_stream_ref : fs :: FSEventStreamRef , info : * mut libc :: c_void , num_events : libc :: size_t , event_paths : * mut libc :: c_void , event_flags : * const fs :: FSEventStreamEventFlags , _event_ids : * const fs :: FSEventStreamEventId ,) { let event_paths = event_paths as * const * const libc :: c_char ; let info = info as * const StreamContextInfo ; let event_handler = & (* info) . event_handler ; for p in 0 .. num_events { let path = CStr :: from_ptr (* event_paths . add (p)) . to_str () . expect ("Invalid UTF8 string.") ; let path = PathBuf :: from (path) ; let flag = * event_flags . add (p) ; let flag = StreamFlags :: from_bits (flag) . unwrap_or_else (| | { panic ! ("Unable to decode StreamFlags: {}" , flag) ; }) ; let mut handle_event = false ; for (p , r) in & (* info) . recursive_info { if path . starts_with (p) { if * r || & path == p { handle_event = true ; break ; } else if let Some (parent_path) = path . parent () { if parent_path == p { handle_event = true ; break ; } } } } if ! handle_event { continue ; } log :: trace ! ("FSEvent: path = `{}`, flag = {:?}" , path . display () , flag) ; for ev in translate_flags (flag , true) . into_iter () { let ev = ev . add_path (path . clone ()) ; let mut event_handler = event_handler . lock () . expect ("lock not to be poisoned") ; event_handler . handle_event (Ok (ev)) ; } } }
};
}
