macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "8")] impl < Context > Decode < Context > for AtomicI8 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI8 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_21!()