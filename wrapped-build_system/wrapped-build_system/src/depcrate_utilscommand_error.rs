// Generated macro for command_error (function)
macro_rules! Depcrate_utilscommand_error {
() => {
// Module: crate::utils
// Provides: {"command_error"}
// Dependencies: {}
fn command_error < D : Debug > (input : & [& dyn AsRef < OsStr >] , cwd : & Option < & Path > , error : D) -> String { format ! ("Command `{}`{} failed to run: {error:?}" , input . iter () . map (| s | s . as_ref () . to_str () . unwrap ()) . collect ::< Vec < _ >> () . join (" ") , cwd . as_ref () . map (| cwd | format ! (" (running in folder `{}`)" , cwd . display () ,)) . unwrap_or_default () ,) }
};
}
