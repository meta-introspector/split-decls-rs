macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < T > Encode for PhantomData < T > { fn encode < E : Encoder > (& self , _ : & mut E) -> Result < () , EncodeError > { Ok (()) } }
    };
}

impl_434!()