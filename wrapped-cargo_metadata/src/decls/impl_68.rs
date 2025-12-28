macro_rules! deps {
    () => {
        Result!();
        TargetKind!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl FromStr for TargetKind { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> std :: result :: Result < Self , Self :: Err > { Ok (TargetKind :: from (s)) } }
    };
}

impl_68!();