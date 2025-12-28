macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < Context > Decode < Context > for Arc < str > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
    };
}

impl_77!()