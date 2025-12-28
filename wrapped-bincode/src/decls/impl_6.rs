macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "8")] impl < Context > Decode < Context > for AtomicU8 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU8 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_6!()