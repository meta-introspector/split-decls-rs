macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl FromMeta for Number { fn from_value (value : & Lit) -> darling :: Result < Self > { match value { Lit :: Int (n) => Ok (Number :: I64 (n . base10_parse :: < i64 > () ?)) , Lit :: Float (n) => Ok (Number :: F64 (n . base10_parse :: < f64 > () ?)) , _ => Err (darling :: Error :: unexpected_type ("number")) , } } }
    };
}

impl_128!();