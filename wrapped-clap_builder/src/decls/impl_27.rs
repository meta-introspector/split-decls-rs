macro_rules! deps {
    () => {
        Result!();
        FromArgMatches!();
        Error!();
        ArgMatches!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl FromArgMatches for () { fn from_arg_matches (_matches : & ArgMatches) -> Result < Self , Error > { Ok (()) } fn update_from_arg_matches (& mut self , _matches : & ArgMatches) -> Result < () , Error > { Ok (()) } }
    };
}

impl_27!()