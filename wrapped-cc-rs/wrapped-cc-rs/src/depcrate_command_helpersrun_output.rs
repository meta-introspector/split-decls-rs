// Generated macro for run_output (function)
macro_rules! Depcrate_command_helpersrun_output {
() => {
// Module: crate::command_helpers
// Provides: {"run_output"}
// Dependencies: {}
pub (crate) fn run_output (cmd : & mut Command , cargo_output : & CargoOutput) -> Result < Vec < u8 > , Error > { let mut captured_cargo_output = cargo_output . clone () ; captured_cargo_output . output = OutputKind :: Capture ; let mut child = spawn (cmd , & captured_cargo_output) ? ; let mut stdout = vec ! [] ; child . stdout . take () . unwrap () . read_to_end (& mut stdout) . unwrap () ; wait_on_child (cmd , & mut child , cargo_output) ? ; Ok (stdout) }
};
}
