macro_rules! deps {
    () => {
        Shell!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl FromStr for Shell { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { for variant in Self :: value_variants () { if variant . to_possible_value () . unwrap () . matches (s , false) { return Ok (* variant) ; } } Err (format ! ("invalid variant: {s}")) } }
    };
}

impl_50!()