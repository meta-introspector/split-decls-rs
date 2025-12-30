// Generated macro for with_command (function)
macro_rules! Depcratewith_command {
() => {
// Module: crate
// Provides: {"with_command"}
// Dependencies: {}
# [doc = " Get a command that uses `app` to open `path`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn main() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " let path = \"http://rust-lang.org\";"] # [doc = " assert!(open::with_command(path, \"app\").status()?.success());"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] pub fn with_command (path : impl AsRef < OsStr > , app : impl Into < String >) -> Command { os :: with_command (path , app) }
};
}
