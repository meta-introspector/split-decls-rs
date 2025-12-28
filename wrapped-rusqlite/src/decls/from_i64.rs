macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! from_i64 {
    () => {
        deps!();
        macro_rules ! from_i64 (($ t : ty) => (impl From <$ t > for Value { # [inline] fn from (i : $ t) -> Value { Value :: Integer (i64 :: from (i)) } })) ;
    };
}

from_i64!();