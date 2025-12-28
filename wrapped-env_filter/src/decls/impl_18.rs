macro_rules! deps {
    () => {
        FilterOp!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [cfg (not (feature = "regex"))] impl FilterOp { pub fn new (spec : & str) -> Result < Self , String > { Ok (Self { inner : spec . to_string () , }) } pub fn is_match (& self , s : & str) -> bool { s . contains (& self . inner) } }
    };
}

impl_18!()