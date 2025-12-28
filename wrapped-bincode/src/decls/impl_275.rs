macro_rules! deps {
    () => {
        BorrowDecoder!();
        DecoderImpl!();
        Config!();
        BorrowReader!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'de , R : BorrowReader < 'de > , C : Config , Context > BorrowDecoder < 'de > for DecoderImpl < R , C , Context > { type BR = R ; fn borrow_reader (& mut self) -> & mut Self :: BR { & mut self . reader } }
    };
}

impl_275!();