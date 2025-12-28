macro_rules! deps {
    () => {
        IoReader!();
        Config!();
        OwnedSerdeDecoder!();
        DecoderImpl!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'r , C : Config , R : std :: io :: Read > OwnedSerdeDecoder < DecoderImpl < IoReader < & 'r mut R > , C , () > > { # [doc = " Creates the decoder from an `std::io::Read` implementor."] pub fn from_std_read (src : & 'r mut R , config : C ,) -> OwnedSerdeDecoder < DecoderImpl < IoReader < & 'r mut R > , C , () > > where C : Config , { let reader = IoReader :: new (src) ; let decoder = DecoderImpl :: new (reader , config , ()) ; Self { de : decoder } } }
    };
}

impl_150!();