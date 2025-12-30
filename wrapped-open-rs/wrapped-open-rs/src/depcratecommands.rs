// Generated macro for commands (function)
macro_rules! Depcratecommands {
() => {
// Module: crate
// Provides: {"commands"}
// Dependencies: {}
# [doc = " Get multiple commands that open `path` with the default application."] # [doc = ""] # [doc = " Each command represents a launcher to try."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn main() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " let path = \"http://rust-lang.org\";"] # [doc = " assert!(open::commands(path)[0].status()?.success());"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] pub fn commands (path : impl AsRef < OsStr >) -> Vec < Command > { os :: commands (path) }
};
}
