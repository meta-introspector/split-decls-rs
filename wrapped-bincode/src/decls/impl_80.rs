macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < T > Encode for Arc < T > where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
    };
}

impl_80!();