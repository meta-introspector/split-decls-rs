macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < T > Encode for Mutex < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let t = self . lock () . map_err (| _ | EncodeError :: LockFailed { type_name : core :: any :: type_name :: < Mutex < T > > () , }) ? ; t . encode (encoder) } }
    };
}

impl_98!();