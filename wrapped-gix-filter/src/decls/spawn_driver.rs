macro_rules! deps {
    () => {
        Error!();
        Context!();
    };
}

macro_rules! spawn_driver {
    () => {
        deps!();
        fn spawn_driver (cmd : BString , context : & gix_command :: Context ,) -> Result < (std :: process :: Child , std :: process :: Command) , Error > { let mut cmd : std :: process :: Command = gix_command :: prepare (gix_path :: from_bstr (cmd) . into_owned ()) . command_may_be_shell_script () . with_context (context . clone ()) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: inherit ()) . into () ; gix_trace :: debug ! (cmd = ? cmd , "launching filter driver") ; let child = match cmd . spawn () { Ok (child) => child , Err (err) => { return Err (Error :: SpawnCommand { source : err , command : cmd , }) } } ; Ok ((child , cmd)) }
    };
}

spawn_driver!();