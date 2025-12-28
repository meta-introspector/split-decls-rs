macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < Context > Decode < Context > for AtomicUsize { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicUsize :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_18!();