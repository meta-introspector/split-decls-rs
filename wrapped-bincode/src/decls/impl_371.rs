macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for core :: marker :: PhantomData < T > { fn decode < D : Decoder < Context = Context > > (_ : & mut D) -> Result < Self , DecodeError > { Ok (core :: marker :: PhantomData) } }
    };
}

impl_371!();