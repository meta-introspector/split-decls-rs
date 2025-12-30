// Generated macro for command_add_output_file (function)
macro_rules! Depcrate_command_helperscommand_add_output_file {
() => {
// Module: crate::command_helpers
// Provides: {"command_add_output_file"}
// Dependencies: {}
pub (crate) fn command_add_output_file (cmd : & mut Command , dst : & Path , args : CmdAddOutputFileArgs) { if args . is_assembler_msvc || ! (! args . msvc || args . clang || args . gnu || args . cuda || (args . is_asm && args . is_arm)) { let mut s = OsString :: from ("-Fo") ; s . push (dst) ; cmd . arg (s) ; } else { cmd . arg ("-o") . arg (dst) ; } }
};
}
