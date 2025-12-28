macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "16")] impl < Context > Decode < Context > for AtomicI16 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI16 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_24!();