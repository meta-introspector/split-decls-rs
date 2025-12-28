macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < Context > Decode < Context > for SystemTime { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let duration = Duration :: decode (decoder) ? ; match SystemTime :: UNIX_EPOCH . checked_add (duration) { Some (t) => Ok (t) , None => Err (DecodeError :: InvalidSystemTime { duration }) , } } }
    };
}

impl_105!();