// Generated macro for stop_watch (function)
macro_rules! Depcrate_windowsstop_watch {
() => {
// Module: crate::windows
// Provides: {"stop_watch"}
// Dependencies: {}
fn stop_watch (ws : & WatchState , meta_tx : & Sender < MetaEvent >) { unsafe { let cio = CancelIo (ws . dir_handle) ; let ch = CloseHandle (ws . dir_handle) ; if cio != 0 && ch != 0 { while WaitForSingleObjectEx (ws . complete_sem , INFINITE , 1) != WAIT_OBJECT_0 { } } CloseHandle (ws . complete_sem) ; } let _ = meta_tx . send (MetaEvent :: SingleWatchComplete) ; }
};
}
