macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "64")] impl < Context > Decode < Context > for AtomicI64 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI64 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_30!()