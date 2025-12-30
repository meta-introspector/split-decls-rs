// Generated macro for get_key_event_count (function)
macro_rules! Depcrate_windows_termget_key_event_count {
() => {
// Module: crate::windows_term
// Provides: {"get_key_event_count"}
// Dependencies: {}
# [doc = " Get the number of pending events in the ReadConsoleInput queue. Note that while"] # [doc = " these aren't necessarily key events, the only way that multiple events can be"] # [doc = " put into the queue simultaneously is if a unicode character spanning multiple u16's"] # [doc = " is read."] # [doc = ""] # [doc = " Therefore, this is accurate as long as at least one KEY_EVENT has already been read."] fn get_key_event_count () -> io :: Result < u32 > { let handle = get_stdin_handle () ? ; let mut event_count : u32 = unsafe { mem :: zeroed () } ; let success = unsafe { GetNumberOfConsoleInputEvents (handle , & mut event_count) } ; if success == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (event_count) } }
};
}
