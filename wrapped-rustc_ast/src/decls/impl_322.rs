macro_rules! deps {
    () => {
        DiffMode!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl FromStr for DiffMode { type Err = () ; fn from_str (s : & str) -> Result < DiffMode , () > { match s { "Error" => Ok (DiffMode :: Error) , "Source" => Ok (DiffMode :: Source) , "Forward" => Ok (DiffMode :: Forward) , "Reverse" => Ok (DiffMode :: Reverse) , _ => Err (()) , } } }
    };
}

impl_322!();