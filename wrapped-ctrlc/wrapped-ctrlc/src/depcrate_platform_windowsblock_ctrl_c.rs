// Generated macro for block_ctrl_c (function)
macro_rules! Depcrate_platform_windowsblock_ctrl_c {
() => {
// Module: crate::platform::windows
// Provides: {"block_ctrl_c"}
// Dependencies: {}
# [doc = " Blocks until a Ctrl-C signal is received."] # [doc = ""] # [doc = " Must be called after calling [`init_os_handler()`](fn.init_os_handler.html)."] # [doc = ""] # [doc = " # Errors"] # [doc = " Will return an error if a system error occurred."] # [doc = ""] # [inline] pub unsafe fn block_ctrl_c () -> Result < () , Error > { match WaitForSingleObject (SEMAPHORE , INFINITE) { WAIT_OBJECT_0 => Ok (()) , WAIT_FAILED => Err (io :: Error :: last_os_error ()) , ret => Err (io :: Error :: new (io :: ErrorKind :: Other , format ! ("WaitForSingleObject(), unexpected return value \"{:x}\"" , ret) ,)) , } }
};
}
