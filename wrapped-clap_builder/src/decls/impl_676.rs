macro_rules! deps {
    () => {
        Result!();
        ColorChoice!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl std :: str :: FromStr for ColorChoice { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { for variant in Self :: value_variants () { if variant . to_possible_value () . unwrap () . matches (s , false) { return Ok (* variant) ; } } Err (format ! ("invalid variant: {s}")) } }
    };
}

impl_676!()