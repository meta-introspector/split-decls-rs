macro_rules! deps {
    () => {
        BorrowDecode!();
        SerdeDecoder!();
        BorrowDecoder!();
        BorrowCompat!();
        DecodeError!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'de , T , Context > crate :: de :: BorrowDecode < 'de , Context > for BorrowCompat < T > where T : serde :: de :: Deserialize < 'de > , { fn borrow_decode < D : crate :: de :: BorrowDecoder < 'de > > (decoder : & mut D ,) -> Result < Self , crate :: error :: DecodeError > { let serde_decoder = de_borrowed :: SerdeDecoder { de : decoder , pd : core :: marker :: PhantomData , } ; T :: deserialize (serde_decoder) . map (BorrowCompat) } }
    };
}

impl_191!();