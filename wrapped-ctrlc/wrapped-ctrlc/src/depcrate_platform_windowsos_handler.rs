// Generated macro for os_handler (function)
macro_rules! Depcrate_platform_windowsos_handler {
() => {
// Module: crate::platform::windows
// Provides: {"os_handler"}
// Dependencies: {}
unsafe extern "system" fn os_handler (_ : u32) -> BOOL { ReleaseSemaphore (SEMAPHORE , 1 , ptr :: null_mut ()) ; TRUE }
};
}
