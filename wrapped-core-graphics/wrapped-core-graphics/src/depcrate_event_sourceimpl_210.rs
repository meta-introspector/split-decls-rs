// Generated macro for impl_210 (impl)
macro_rules! Depcrate_event_sourceimpl_210 {
() => {
// Module: crate::event_source
// Provides: {"impl_210"}
// Dependencies: {}
impl CGEventSource { pub fn type_id () -> CFTypeID { unsafe { CGEventSourceGetTypeID () } } pub fn new (state_id : CGEventSourceStateID) -> Result < Self , () > { unsafe { let event_source_ref = CGEventSourceCreate (state_id) ; if ! event_source_ref . is_null () { Ok (Self :: from_ptr (event_source_ref)) } else { Err (()) } } } }
};
}
