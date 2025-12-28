macro_rules! deps {
    () => {
        CrateType!();
        Result!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl FromStr for CrateType { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> std :: result :: Result < Self , Self :: Err > { Ok (CrateType :: from (s)) } }
    };
}

impl_72!()