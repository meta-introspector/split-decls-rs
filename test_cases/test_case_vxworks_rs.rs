// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/process/unix/vxworks.rs
// Error: expected square brackets
// Problematic line: line 15

// Command
////////////////////////////////////////////////////////////////////////////////

impl Command {
    pub fn spawn(
        &mut self,
        default: Stdio,
