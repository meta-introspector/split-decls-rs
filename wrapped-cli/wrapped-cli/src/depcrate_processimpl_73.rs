// Generated macro for impl_73 (impl)
macro_rules! Depcrate_processimpl_73 {
() => {
// Module: crate::process
// Provides: {"impl_73"}
// Dependencies: {}
impl StderrReader { # [doc = " Create a reader for stderr that reads contents asynchronously."] fn r#async (mut stderr : process :: ChildStderr) -> StderrReader { let handle = std :: thread :: spawn (move | | stderr_to_command_error (& mut stderr)) ; StderrReader :: Async (Some (handle)) } # [doc = " Create a reader for stderr that reads contents synchronously."] fn sync (stderr : process :: ChildStderr) -> StderrReader { StderrReader :: Sync (stderr) } # [doc = " Consumes all of stderr on to the heap and returns it as an error."] # [doc = ""] # [doc = " If there was a problem reading stderr itself, then this returns an I/O"] # [doc = " command error."] fn read_to_end (& mut self) -> CommandError { match * self { StderrReader :: Async (ref mut handle) => { let handle = handle . take () . expect ("read_to_end cannot be called more than once") ; handle . join () . expect ("stderr reading thread does not panic") } StderrReader :: Sync (ref mut stderr) => { stderr_to_command_error (stderr) } } } }
};
}
