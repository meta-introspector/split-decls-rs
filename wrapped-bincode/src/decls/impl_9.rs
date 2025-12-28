macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "16")] impl < Context > Decode < Context > for AtomicU16 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU16 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_9!()