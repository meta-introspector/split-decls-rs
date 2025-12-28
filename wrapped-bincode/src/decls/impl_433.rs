macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl Encode for () { fn encode < E : Encoder > (& self , _ : & mut E) -> Result < () , EncodeError > { Ok (()) } }
    };
}

impl_433!();