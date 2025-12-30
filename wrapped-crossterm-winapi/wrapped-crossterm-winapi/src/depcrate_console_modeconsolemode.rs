// Generated macro for ConsoleMode (struct)
macro_rules! Depcrate_console_modeConsoleMode {
() => {
// Module: crate::console_mode
// Provides: {"ConsoleMode"}
// Dependencies: {}
# [doc = " A wrapper around a screen buffer, focusing on calls to get and set the console mode."] # [doc = ""] # [doc = " This wraps [`SetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/setconsolemode)"] # [doc = " and [`GetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/getconsolemode)."] # [derive (Debug , Clone)] pub struct ConsoleMode { handle : Handle , }
};
}
