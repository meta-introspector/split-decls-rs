// Generated macro for execute_fmt (function)
macro_rules! Depcrate_commandexecute_fmt {
() => {
// Module: crate::command
// Provides: {"execute_fmt"}
// Dependencies: {}
# [doc = " Executes the ANSI representation of a command, using the given `fmt::Write`."] pub (crate) fn execute_fmt (f : & mut impl fmt :: Write , command : impl Command) -> fmt :: Result { # [cfg (windows)] if ! command . is_ansi_code_supported () { return command . execute_winapi () . map_err (| _ | fmt :: Error) ; } command . write_ansi (f) }
};
}
