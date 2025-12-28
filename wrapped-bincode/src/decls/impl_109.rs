macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Encode for PathBuf { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_path () . encode (encoder) } }
    };
}

impl_109!()