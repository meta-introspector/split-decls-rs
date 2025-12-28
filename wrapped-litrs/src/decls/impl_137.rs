macro_rules! deps {
    () => {
        FloatType!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl FromStr for FloatType { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from_suffix (s) . ok_or (()) } }
    };
}

impl_137!()