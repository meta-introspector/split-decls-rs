macro_rules! deps {
    () => {
        Reader!();
        DecoderImpl!();
        OwnedSerdeDecoder!();
        Config!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < C : Config , R : Reader > OwnedSerdeDecoder < DecoderImpl < R , C , () > > { # [doc = " Creates the decoder from a [`Reader`] implementor."] pub fn from_reader (reader : R , config : C) -> OwnedSerdeDecoder < DecoderImpl < R , C , () > > where C : Config , { let decoder = DecoderImpl :: new (reader , config , ()) ; Self { de : decoder } } }
    };
}

impl_151!()