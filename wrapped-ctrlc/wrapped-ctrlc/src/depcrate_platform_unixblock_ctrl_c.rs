// Generated macro for block_ctrl_c (function)
macro_rules! Depcrate_platform_unixblock_ctrl_c {
() => {
// Module: crate::platform::unix
// Provides: {"block_ctrl_c"}
// Dependencies: {}
# [doc = " Blocks until a Ctrl-C signal is received."] # [doc = ""] # [doc = " Must be called after calling [`init_os_handler()`](fn.init_os_handler.html)."] # [doc = ""] # [doc = " # Errors"] # [doc = " None."] # [doc = ""] # [inline] pub unsafe fn block_ctrl_c () -> Result < () , CtrlcError > { implementation :: sem_wait_forever () ; Ok (()) }
};
}
