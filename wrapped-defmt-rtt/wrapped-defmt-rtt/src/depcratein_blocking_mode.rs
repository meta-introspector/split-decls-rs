// Generated macro for in_blocking_mode (function)
macro_rules! Depcratein_blocking_mode {
() => {
// Module: crate
// Provides: {"in_blocking_mode"}
// Dependencies: {}
# [doc = " Report whether the SEGGER RTT up channel is in blocking mode."] # [doc = ""] # [doc = " Returns true if the mode bitfield within the flags value has been set to"] # [doc = " `SEGGER_RTT_MODE_BLOCK_IF_FIFO_FULL`."] # [doc = ""] # [doc = " Currently we start-up in non-blocking mode, so if it's been set to blocking"] # [doc = " mode then the connected client (e.g. probe-rs) must have done it."] pub fn in_blocking_mode () -> bool { (_SEGGER_RTT . up_channel . flags . load (Ordering :: Relaxed) & MODE_MASK) == MODE_BLOCK_IF_FULL }
};
}
