macro_rules! deps {
    () => {
        Suffix!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl FromStr for Suffix { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "k" | "K" => Ok (Self :: Kibi) , "m" | "M" => Ok (Self :: Mebi) , "g" | "G" => Ok (Self :: Gibi) , _ => Err (()) , } } }
    };
}

impl_38!()