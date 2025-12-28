macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < Context > Decode < Context > for bool { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u8 :: decode (decoder) ? { 0 => Ok (false) , 1 => Ok (true) , x => Err (DecodeError :: InvalidBooleanValue (x)) , } } }
    };
}

impl_305!()