// Generated macro for impl_34 (impl)
macro_rules! Depcrate_console_modeimpl_34 {
() => {
// Module: crate::console_mode
// Provides: {"impl_34"}
// Dependencies: {}
impl ConsoleMode { # [doc = " Create a new `ConsoleMode` instance."] # [doc = ""] # [doc = " This will use the standard output as its handle."] # [doc = " When you explicitly want to specify the handle used for the function calls use `ConsoleMode::from(handle)` instead."] pub fn new () -> Result < ConsoleMode > { Ok (ConsoleMode { handle : Handle :: new (HandleType :: OutputHandle) ? , }) } # [doc = " Set the console mode to the given console mode."] # [doc = ""] # [doc = " This function sets the `dwMode`."] # [doc = ""] # [doc = " This wraps"] # [doc = " [`SetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/setconsolemode)."] pub fn set_mode (& self , console_mode : u32) -> Result < () > { result (unsafe { SetConsoleMode (* self . handle , console_mode) }) } # [doc = " Get the console mode."] # [doc = ""] # [doc = " This function returns the `lpMode`."] # [doc = ""] # [doc = " This wraps"] # [doc = " [`GetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/getconsolemode)."] pub fn mode (& self) -> Result < u32 > { let mut console_mode = 0 ; result (unsafe { GetConsoleMode (* self . handle , & mut console_mode) }) ? ; Ok (console_mode) } }
};
}
