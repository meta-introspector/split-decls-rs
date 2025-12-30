// Generated macro for ProcessBuilder (struct)
macro_rules! Depcrate_process_builderProcessBuilder {
() => {
// Module: crate::process_builder
// Provides: {"ProcessBuilder"}
// Dependencies: {}
# [doc = " A builder object for an external process, similar to [`std::process::Command`]."] # [derive (Clone , Debug)] pub struct ProcessBuilder { # [doc = " The program to execute."] program : OsString , # [doc = " Best-effort replacement for arg0"] arg0 : Option < OsString > , # [doc = " A list of arguments to pass to the program."] args : Vec < OsString > , # [doc = " Any environment variables that should be set for the program."] env : BTreeMap < String , Option < OsString > > , # [doc = " The directory to run the program from."] cwd : Option < OsString > , # [doc = " A list of wrappers that wrap the original program when calling"] # [doc = " [`ProcessBuilder::wrapped`]. The last one is the outermost one."] wrappers : Vec < OsString > , # [doc = " The `make` jobserver. See the [jobserver crate] for"] # [doc = " more information."] # [doc = ""] # [doc = " [jobserver crate]: https://docs.rs/jobserver/"] jobserver : Option < Client > , # [doc = " `true` to include environment variable in display."] display_env_vars : bool , # [doc = " `true` to retry with an argfile if hitting \"command line too big\" error."] # [doc = " See [`ProcessBuilder::retry_with_argfile`] for more information."] retry_with_argfile : bool , # [doc = " Data to write to stdin."] stdin : Option < Vec < u8 > > , }
};
}
