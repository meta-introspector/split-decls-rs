macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl FromStr for Endian { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "little" => Ok (Self :: Little) , "big" => Ok (Self :: Big) , _ => Err (format ! (r#"unknown endian: "{s}""#)) , } } }
    };
}

impl_25!()