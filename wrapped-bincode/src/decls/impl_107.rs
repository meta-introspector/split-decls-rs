macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl Encode for & '_ Path { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self . to_str () { Some (str) => str . encode (encoder) , None => Err (EncodeError :: InvalidPathCharacters) , } } }
    };
}

impl_107!()