macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        pub struct SerializeTupleVariant { name : String , vec : Vec < Value > , }
    };
}

SerializeTupleVariant!();