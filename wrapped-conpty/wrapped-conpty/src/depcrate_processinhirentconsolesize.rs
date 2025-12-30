// Generated macro for inhirentConsoleSize (function)
macro_rules! Depcrate_processinhirentConsoleSize {
() => {
// Module: crate::process
// Provides: {"inhirentConsoleSize"}
// Dependencies: {}
fn inhirentConsoleSize () -> win :: Result < COORD > { let stdout_h = stdout_handle () ? ; let mut info = CONSOLE_SCREEN_BUFFER_INFO :: default () ; unsafe { GetConsoleScreenBufferInfo (stdout_h , & mut info) ? ; CloseHandle (stdout_h) ? ; } ; let mut size = COORD { X : 24 , Y : 80 } ; size . X = info . srWindow . Right - info . srWindow . Left + 1 ; size . Y = info . srWindow . Bottom - info . srWindow . Top + 1 ; Ok (size) }
};
}
