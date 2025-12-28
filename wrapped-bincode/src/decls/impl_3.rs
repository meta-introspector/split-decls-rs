macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "8")] impl < Context > Decode < Context > for AtomicBool { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicBool :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_3!()