macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! validate {
    () => {
        deps!();
        fn validate (key : & str , value : & BStr) -> Result < () , Error > { if key . contains ('\0') || key . contains ('\n') || value . contains (& 0) || value . contains (& b'\n') { return Err (Error :: Encoding { key : key . to_owned () , value : value . to_owned () , }) ; } Ok (()) }
    };
}

validate!()