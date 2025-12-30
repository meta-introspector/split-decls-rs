// Generated macro for tests (module)
macro_rules! Depcrate_screen_buffertests {
() => {
// Module: crate::screen_buffer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ScreenBuffer ; # [test] fn test_screen_buffer_info () { let buffer = ScreenBuffer :: current () . unwrap () ; let info = buffer . info () . unwrap () ; info . terminal_size () ; info . terminal_window () ; info . attributes () ; info . cursor_pos () ; } }
};
}
