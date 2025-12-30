// Generated macro for maybe_dump (function)
macro_rules! Depcrate_utils_shared_helpersmaybe_dump {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"maybe_dump"}
// Dependencies: {}
# [doc = " Writes the command invocation to a file if `DUMP_BOOTSTRAP_SHIMS` is set during bootstrap."] # [doc = ""] # [doc = " Before writing it, replaces user-specific values to create generic dumps for cross-environment"] # [doc = " comparisons."] pub fn maybe_dump (dump_name : String , cmd : & Command) { if let Ok (dump_dir) = env :: var ("DUMP_BOOTSTRAP_SHIMS") { let dump_file = format ! ("{dump_dir}/{dump_name}") ; let mut file = OpenOptions :: new () . create (true) . append (true) . open (dump_file) . unwrap () ; let cmd_dump = format ! ("{cmd:?}\n") ; let cmd_dump = cmd_dump . replace (& env :: var ("BUILD_OUT") . unwrap () , "${BUILD_OUT}") ; let cmd_dump = cmd_dump . replace (& env :: var ("CARGO_HOME") . unwrap () , "${CARGO_HOME}") ; file . write_all (cmd_dump . as_bytes ()) . expect ("Unable to write file") ; } }
};
}
