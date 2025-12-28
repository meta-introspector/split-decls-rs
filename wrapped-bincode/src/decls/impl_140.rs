macro_rules! deps {
    () => {
        DecoderImpl!();
        Config!();
        BorrowedSerdeDecoder!();
        SliceReader!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'de , C : Config , Context > BorrowedSerdeDecoder < 'de , DecoderImpl < SliceReader < 'de > , C , Context > > { # [doc = " Creates the decoder from a borrowed slice."] pub fn from_slice (slice : & 'de [u8] , config : C , context : Context ,) -> BorrowedSerdeDecoder < 'de , DecoderImpl < SliceReader < 'de > , C , Context > > where C : Config , { let reader = SliceReader :: new (slice) ; let decoder = DecoderImpl :: new (reader , config , context) ; Self { de : decoder , pd : PhantomData , } } }
    };
}

impl_140!();