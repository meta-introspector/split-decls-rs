macro_rules! deps {
    () => {
        TargetKind!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl FromStr for TargetKind { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> std :: result :: Result < Self , Self :: Err > { Ok (TargetKind :: from (s)) } }
    };
}

impl_33!()