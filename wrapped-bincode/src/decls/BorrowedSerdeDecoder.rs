macro_rules! deps {
    () => {
        BorrowDecoder!();
    };
}

macro_rules! BorrowedSerdeDecoder {
    () => {
        deps!();
        # [doc = " Serde decoder encapsulating a borrowed reader."] pub struct BorrowedSerdeDecoder < 'de , DE : BorrowDecoder < 'de > > { pub (super) de : DE , pub (super) pd : PhantomData < & 'de () > , }
    };
}

BorrowedSerdeDecoder!();