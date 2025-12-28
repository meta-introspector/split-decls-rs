macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < Context , T > Decode < Context > for Arc < [T] > where T : Decode < Context > + 'static , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = Vec :: decode (decoder) ? ; Ok (vec . into ()) } }
    };
}

impl_81!();