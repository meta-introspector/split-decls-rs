// Generated macro for with_threadlocal_overlapped (function)
macro_rules! Depcrate_pipewith_threadlocal_overlapped {
() => {
// Module: crate::pipe
// Provides: {"with_threadlocal_overlapped"}
// Dependencies: {}
# [doc = " Call a function with a threadlocal `Overlapped`.  The function `f` should be"] # [doc = " sure that the event is reset, either manually or by a thread being released."] fn with_threadlocal_overlapped < F > (f : F) -> io :: Result < usize > where F : FnOnce (& Overlapped) -> io :: Result < usize > , { NAMED_PIPE_OVERLAPPED . with (| overlapped | { let mut mborrow = overlapped . borrow_mut () ; if mborrow . is_none () { let op = Overlapped :: initialize_with_autoreset_event () ? ; * mborrow = Some (op) ; } f (mborrow . as_ref () . unwrap ()) }) }
};
}
