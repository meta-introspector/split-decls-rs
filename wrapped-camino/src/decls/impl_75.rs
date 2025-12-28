macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl FromStr for Utf8PathBuf { type Err = Infallible ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Utf8PathBuf (s . into ())) } }
    };
}

impl_75!();