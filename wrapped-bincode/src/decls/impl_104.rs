macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Encode for SystemTime { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let duration = self . duration_since (SystemTime :: UNIX_EPOCH) . map_err (| e | { EncodeError :: InvalidSystemTime { inner : e , time : std :: boxed :: Box :: new (* self) , } }) ? ; duration . encode (encoder) } }
    };
}

impl_104!()