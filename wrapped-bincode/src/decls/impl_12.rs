macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "32")] impl < Context > Decode < Context > for AtomicU32 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU32 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_12!();