macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "64")] impl < Context > Decode < Context > for AtomicU64 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU64 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_15!()