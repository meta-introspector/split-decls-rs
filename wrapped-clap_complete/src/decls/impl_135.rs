macro_rules! deps {
    () => {
        Fish!();
        EnvCompleter!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl EnvCompleter for Fish { fn name (& self) -> & 'static str { "fish" } fn is (& self , name : & str) -> bool { name == "fish" } fn write_registration (& self , var : & str , _name : & str , bin : & str , completer : & str , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let bin = shlex :: try_quote (bin) . unwrap_or (std :: borrow :: Cow :: Borrowed (bin)) ; let completer = shlex :: try_quote (completer) . unwrap_or (std :: borrow :: Cow :: Borrowed (completer)) ; writeln ! (buf , r#"complete --keep-order --exclusive --command {bin} --arguments "({var}=fish "'{completer}'" -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))""#) } fn write_complete (& self , cmd : & mut clap :: Command , args : Vec < OsString > , current_dir : Option < & std :: path :: Path > , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let index = args . len () - 1 ; let completions = crate :: engine :: complete (cmd , args , index , current_dir) ? ; for candidate in completions { write ! (buf , "{}" , candidate . get_value () . to_string_lossy ()) ? ; if let Some (help) = candidate . get_help () { write ! (buf , "\t{}" , help . to_string () . lines () . next () . unwrap_or_default ()) ? ; } writeln ! (buf) ? ; } Ok (()) } }
    };
}

impl_135!();