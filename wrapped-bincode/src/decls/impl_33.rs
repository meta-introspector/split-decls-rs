macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < Context > Decode < Context > for AtomicIsize { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicIsize :: new (Decode :: decode (decoder) ?)) } }
    };
}

impl_33!();