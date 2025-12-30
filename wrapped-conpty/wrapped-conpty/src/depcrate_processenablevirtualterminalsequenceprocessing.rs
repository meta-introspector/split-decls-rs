// Generated macro for enableVirtualTerminalSequenceProcessing (function)
macro_rules! Depcrate_processenableVirtualTerminalSequenceProcessing {
() => {
// Module: crate::process
// Provides: {"enableVirtualTerminalSequenceProcessing"}
// Dependencies: {}
fn enableVirtualTerminalSequenceProcessing () -> win :: Result < () > { let stdout_h = stdout_handle () ? ; unsafe { let mut mode = CONSOLE_MODE :: default () ; GetConsoleMode (stdout_h , & mut mode) ? ; mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING ; SetConsoleMode (stdout_h , mode) ? ; CloseHandle (stdout_h) ? ; } Ok (()) }
};
}
