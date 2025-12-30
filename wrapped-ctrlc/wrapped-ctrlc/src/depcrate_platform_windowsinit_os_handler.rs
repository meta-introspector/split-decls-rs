// Generated macro for init_os_handler (function)
macro_rules! Depcrate_platform_windowsinit_os_handler {
() => {
// Module: crate::platform::windows
// Provides: {"init_os_handler"}
// Dependencies: {}
# [doc = " Register os signal handler."] # [doc = ""] # [doc = " Must be called before calling [`block_ctrl_c()`](fn.block_ctrl_c.html)"] # [doc = " and should only be called once."] # [doc = ""] # [doc = " # Errors"] # [doc = " Will return an error if a system error occurred."] # [doc = ""] # [inline] pub unsafe fn init_os_handler (_overwrite : bool) -> Result < () , Error > { SEMAPHORE = CreateSemaphoreA (ptr :: null_mut () , 0 , MAX_SEM_COUNT , ptr :: null ()) ; if SEMAPHORE . is_null () { return Err (io :: Error :: last_os_error ()) ; } if SetConsoleCtrlHandler (Some (os_handler) , TRUE) == FALSE { let e = io :: Error :: last_os_error () ; CloseHandle (SEMAPHORE) ; SEMAPHORE = 0 as HANDLE ; return Err (e) ; } Ok (()) }
};
}
