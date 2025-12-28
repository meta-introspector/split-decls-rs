macro_rules! deps {
    () => {
        Number!();
        Deserializer!();
        Result!();
        Error!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl FromStr for Number { type Err = Error ; fn from_str (s : & str) -> result :: Result < Self , Self :: Err > { Deserializer :: from_str (s) . parse_any_signed_number () . map (Into :: into) } }
    };
}

impl_21!()