// Generated macro for exec_command (function)
macro_rules! Depcrate_utilsexec_command {
() => {
// Module: crate::utils
// Provides: {"exec_command"}
// Dependencies: {}
fn exec_command (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : Option < & HashMap < String , String > > ,) -> Result < ExitStatus , String > { let status = get_command_inner (input , cwd , env) . spawn () . map_err (| e | command_error (input , & cwd , e)) ? . wait () . map_err (| e | command_error (input , & cwd , e)) ? ; # [cfg (unix)] { if let Some (signal) = status . signal () { return Err (command_error (input , & cwd , format ! ("Process received signal {signal}"))) ; } } Ok (status) }
};
}
