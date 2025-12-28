macro_rules! deps {
    () => {
        CrateType!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl FromStr for CrateType { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> std :: result :: Result < Self , Self :: Err > { Ok (CrateType :: from (s)) } }
    };
}

impl_37!()