// Generated macro for read_key_event (function)
macro_rules! Depcrate_windows_termread_key_event {
() => {
// Module: crate::windows_term
// Provides: {"read_key_event"}
// Dependencies: {}
fn read_key_event () -> io :: Result < KEY_EVENT_RECORD > { let handle = get_stdin_handle () ? ; let mut buffer : INPUT_RECORD = unsafe { mem :: zeroed () } ; let mut events_read : u32 = unsafe { mem :: zeroed () } ; let mut key_event : KEY_EVENT_RECORD ; loop { let success = unsafe { ReadConsoleInputW (handle , & mut buffer , 1 , & mut events_read) } ; if success == 0 { return Err (io :: Error :: last_os_error ()) ; } if events_read == 0 { return Err (io :: Error :: new (io :: ErrorKind :: Other , "ReadConsoleInput returned no events, instead of waiting for an event" ,)) ; } if events_read == 1 && buffer . EventType != KEY_EVENT as u16 { continue ; } key_event = unsafe { mem :: transmute :: < INPUT_RECORD_0 , KEY_EVENT_RECORD > (buffer . Event) } ; if key_event . bKeyDown == 0 { continue ; } return Ok (key_event) ; } }
};
}
