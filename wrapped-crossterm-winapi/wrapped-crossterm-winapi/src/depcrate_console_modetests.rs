// Generated macro for tests (module)
macro_rules! Depcrate_console_modetests {
() => {
// Module: crate::console_mode
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ConsoleMode ; # [test] # [ignore] fn test_set_get_mode () { let mode = ConsoleMode :: new () . unwrap () ; let original_mode = mode . mode () . unwrap () ; mode . set_mode (0x0004) . unwrap () ; let console_mode = mode . mode () . unwrap () ; assert_eq ! (console_mode & 0x0004 , mode . mode () . unwrap ()) ; mode . set_mode (original_mode) . unwrap () ; } }
};
}
