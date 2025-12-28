macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "32")] impl < Context > Decode < Context > for AtomicI32 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI32 :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_27!();