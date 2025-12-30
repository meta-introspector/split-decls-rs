// Generated macro for open (function)
macro_rules! Depcrateopen {
() => {
// Module: crate
// Provides: {"open"}
// Dependencies: {}
# [doc = " Opens a file or link with the system default program."] # [doc = ""] # [doc = " Note that a path like \"rustup.rs\" could potentially refer to either a file or a website. If you"] # [doc = " want to open the website, you should add the \"http://\" prefix, for example."] # [doc = ""] # [doc = " Also note that a result of `Ok(())` just means a way of opening the path was found, and no error"] # [doc = " occurred as a direct result of opening the path. Errors beyond that point aren't caught. For"] # [doc = " example, `Ok(())` would be returned even if a file was opened with a program that can't read the"] # [doc = " file, or a dead link was opened in a browser."] # [doc = ""] # [doc = " ## Platform Implementation Details"] # [doc = ""] # [doc = " - On Windows the `ShellExecuteW` Windows API function is used."] # [doc = " - On Mac the system `open` command is used."] # [doc = " - On Windows Subsystem for Linux (WSL), the system `wslview` from [`wslu`] is used if available,"] # [doc = "   otherwise the system `xdg-open` is used, if available."] # [doc = " - On non-WSL Linux and other platforms, the system `xdg-open` script is used if available,"] # [doc = "   otherwise an `xdg-open` script embedded in this library is used."] # [doc = ""] # [doc = " [`wslu`]: https://github.com/wslutilities/wslu/"] pub fn open < P > (path : P) -> Result < () , OpenError > where P : AsRef < OsStr > , { sys :: open (path . as_ref ()) }
};
}
