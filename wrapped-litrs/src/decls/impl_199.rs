macro_rules! deps {
    () => {
        IntegerType!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl FromStr for IntegerType { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from_suffix (s) . ok_or (()) } }
    };
}

impl_199!()