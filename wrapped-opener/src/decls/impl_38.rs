macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Display for OpenError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { OpenError :: Io (_) => { write ! (f , "IO error") ? ; } OpenError :: Spawn { cmds , source : _ } => { write ! (f , "error spawning command(s) '{cmds}'") ? ; } OpenError :: ExitStatus { cmd , status , stderr , } => { write ! (f , "command '{cmd}' did not execute successfully; {status}") ? ; let stderr = stderr . trim () ; if ! stderr . is_empty () { write ! (f , "\ncommand stderr:\n{stderr}") ? ; } } } Ok (()) } }
    };
}

impl_38!();