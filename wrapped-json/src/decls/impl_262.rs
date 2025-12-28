macro_rules! deps {
    () => {
        Value!();
        SeqDeserializer!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl SeqDeserializer { fn new (vec : Vec < Value >) -> Self { SeqDeserializer { iter : vec . into_iter () , } } }
    };
}

impl_262!();