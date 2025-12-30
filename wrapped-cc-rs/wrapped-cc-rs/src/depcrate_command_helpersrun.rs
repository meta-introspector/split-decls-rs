// Generated macro for run (function)
macro_rules! Depcrate_command_helpersrun {
() => {
// Module: crate::command_helpers
// Provides: {"run"}
// Dependencies: {}
pub (crate) fn run (cmd : & mut Command , cargo_output : & CargoOutput) -> Result < () , Error > { let mut child = spawn (cmd , cargo_output) ? ; wait_on_child (cmd , & mut child , cargo_output) }
};
}
