macro_rules! deps {
    () => {
        Prepare!();
    };
}

macro_rules! prepare {
    () => {
        deps!();
        # [doc = " Prepare `cmd` for [spawning][std::process::Command::spawn()] by configuring it with various builder methods."] # [doc = ""] # [doc = " Note that the default IO is configured for typical API usage, that is"] # [doc = ""] # [doc = " - `stdin` is null to prevent blocking unexpectedly on consumption of stdin"] # [doc = " - `stdout` is captured for consumption by the caller"] # [doc = " - `stderr` is inherited to allow the command to provide context to the user"] # [doc = ""] # [doc = " On Windows, terminal Windows will be suppressed automatically."] # [doc = ""] # [doc = " ### Warning"] # [doc = ""] # [doc = " When using this method, be sure that the invoked program doesn't rely on the current working dir and/or"] # [doc = " environment variables to know its context. If so, call instead [`Prepare::with_context()`] to provide"] # [doc = " additional information."] pub fn prepare (cmd : impl Into < OsString >) -> Prepare { Prepare { command : cmd . into () , shell_program : None , context : None , stdin : std :: process :: Stdio :: null () , stdout : std :: process :: Stdio :: piped () , stderr : std :: process :: Stdio :: inherit () , args : Vec :: new () , env : Vec :: new () , use_shell : false , quote_command : false , allow_manual_arg_splitting : cfg ! (windows) , } }
    };
}

prepare!()