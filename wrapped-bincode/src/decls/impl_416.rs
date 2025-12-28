macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl < A > Encode for (A ,) where A : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; Ok (()) } }
    };
}

impl_416!()