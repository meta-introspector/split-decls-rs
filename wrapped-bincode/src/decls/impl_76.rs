macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < Context , T > Decode < Context > for Arc < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Arc :: new (t)) } }
    };
}

impl_76!()