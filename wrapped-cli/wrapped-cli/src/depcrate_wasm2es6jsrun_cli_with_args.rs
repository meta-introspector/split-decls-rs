// Generated macro for run_cli_with_args (function)
macro_rules! Depcrate_wasm2es6jsrun_cli_with_args {
() => {
// Module: crate::wasm2es6js
// Provides: {"run_cli_with_args"}
// Dependencies: {}
pub fn run_cli_with_args < I , T > (args : I) -> anyhow :: Result < () > where I : IntoIterator < Item = T > , T : Into < OsString > + Clone , { let args = match Args :: try_parse_from (args) { Ok (a) => a , Err (e) => match e . kind () { clap :: error :: ErrorKind :: DisplayHelp | clap :: error :: ErrorKind :: DisplayVersion => { print ! ("{e}") ; return Ok (()) ; } _ => bail ! (e) , } , } ; rmain (args) }
};
}
