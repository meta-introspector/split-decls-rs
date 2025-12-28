macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for BinaryHeap < T > where T : Decode < Context > + Ord , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: decode (decoder) ? . into ()) } }
    };
}

impl_40!();