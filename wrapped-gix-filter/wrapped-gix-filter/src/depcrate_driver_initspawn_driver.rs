// Generated macro for spawn_driver (function)
macro_rules! Depcrate_driver_initspawn_driver {
() => {
// Module: crate::driver::init
// Provides: {"spawn_driver"}
// Dependencies: {}
fn spawn_driver (cmd : BString , context : & gix_command :: Context ,) -> Result < (std :: process :: Child , std :: process :: Command) , Error > { let mut cmd : std :: process :: Command = gix_command :: prepare (gix_path :: from_bstr (cmd) . into_owned ()) . command_may_be_shell_script () . with_context (context . clone ()) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: inherit ()) . into () ; gix_trace :: debug ! (cmd = ? cmd , "launching filter driver") ; let child = match cmd . spawn () { Ok (child) => child , Err (err) => { return Err (Error :: SpawnCommand { source : err , command : cmd , }) } } ; Ok ((child , cmd)) }
};
}
