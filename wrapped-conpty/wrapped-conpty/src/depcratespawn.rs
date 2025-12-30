// Generated macro for spawn (function)
macro_rules! Depcratespawn {
() => {
// Module: crate
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawns a command using `cmd.exe`."] pub fn spawn (command : impl AsRef < OsStr >) -> Result < Process , Error > { let mut cmd = OsString :: new () ; cmd . push ("cmd /C ") ; cmd . push (command) ; Process :: spawn (Command :: new (& cmd)) }
};
}
