macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Cow < '_ , T > where T : ToOwned + ? Sized , < T as ToOwned > :: Owned : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = < T as ToOwned > :: Owned :: decode (decoder) ? ; Ok (Cow :: Owned (t)) } }
    };
}

impl_65!();