macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < T > Encode for RwLock < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let t = self . read () . map_err (| _ | EncodeError :: LockFailed { type_name : core :: any :: type_name :: < RwLock < T > > () , }) ? ; t . encode (encoder) } }
    };
}

impl_101!()