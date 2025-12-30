// Generated macro for check_exit_status (function)
macro_rules! Depcrate_utilscheck_exit_status {
() => {
// Module: crate::utils
// Provides: {"check_exit_status"}
// Dependencies: {}
fn check_exit_status (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , exit_status : ExitStatus , output : Option < & Output > , show_err : bool ,) -> Result < () , String > { if exit_status . success () { return Ok (()) ; } let mut error = format ! ("Command `{}`{} exited with status {:?}" , input . iter () . map (| s | s . as_ref () . to_str () . unwrap ()) . collect ::< Vec < _ >> () . join (" ") , cwd . map (| cwd | format ! (" (running in folder `{}`)" , cwd . display ())) . unwrap_or_default () , exit_status . code ()) ; let input = input . iter () . map (| i | i . as_ref ()) . collect :: < Vec < & OsStr > > () ; if show_err { eprintln ! ("Command `{input:?}` failed") ; } if let Some (output) = output { let stdout = String :: from_utf8_lossy (& output . stdout) ; if ! stdout . is_empty () { error . push_str ("\n==== STDOUT ====\n") ; error . push_str (& stdout) ; } let stderr = String :: from_utf8_lossy (& output . stderr) ; if ! stderr . is_empty () { error . push_str ("\n==== STDERR ====\n") ; error . push_str (& stderr) ; } } Err (error) }
};
}
