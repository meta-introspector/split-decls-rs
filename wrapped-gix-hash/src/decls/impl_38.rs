macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl FromStr for Kind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "sha1" | "SHA1" => Kind :: Sha1 , other => return Err (other . into ()) , }) } }
    };
}

impl_38!();