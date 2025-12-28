macro_rules! deps {
    () => {
        CompType!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl FromStr for CompType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "9" => Ok (Self :: Normal) , "63" => Ok (Self :: Successive) , "33" => Ok (Self :: Alternatives) , "64" => Ok (Self :: Unmodified) , "37" => Ok (Self :: Menu) , _ => Err (format ! ("unsupported COMP_TYPE `{s}`")) , } } }
    };
}

impl_130!();