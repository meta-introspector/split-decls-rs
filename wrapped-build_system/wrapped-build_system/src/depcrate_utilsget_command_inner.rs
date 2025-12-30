// Generated macro for get_command_inner (function)
macro_rules! Depcrate_utilsget_command_inner {
() => {
// Module: crate::utils
// Provides: {"get_command_inner"}
// Dependencies: {}
pub (crate) fn get_command_inner (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : Option < & HashMap < String , String > > ,) -> Command { let (cmd , args) = match input { [] => panic ! ("empty command") , [cmd , args @ ..] => (cmd , args) , } ; let mut command = Command :: new (cmd) ; command . args (args) ; if let Some (cwd) = cwd { command . current_dir (cwd) ; } if let Some (env) = env { command . envs (env . iter () . map (| (k , v) | (k . as_str () , v . as_str ()))) ; } command }
};
}
