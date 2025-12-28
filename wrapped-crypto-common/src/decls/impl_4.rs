macro_rules! deps {
    () => {
        DeserializeStateError!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Display for DeserializeStateError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("Deserialization error") } }
    };
}

impl_4!()