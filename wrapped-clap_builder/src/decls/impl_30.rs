macro_rules! deps {
    () => {
        ArgMatches!();
        ErrorKind!();
        FromArgMatches!();
        Error!();
        Result!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl FromArgMatches for Infallible { fn from_arg_matches (_matches : & ArgMatches) -> Result < Self , Error > { Err (Error :: raw (crate :: error :: ErrorKind :: MissingSubcommand , "a subcommand is required but one was not provided" ,)) } fn update_from_arg_matches (& mut self , _matches : & ArgMatches) -> Result < () , Error > { unreachable ! ("there will never be an instance of Infallible and thus &mut self can never be called") ; } }
    };
}

impl_30!()