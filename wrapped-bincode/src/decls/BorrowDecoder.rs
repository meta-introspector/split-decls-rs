macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        BorrowReader!();
    };
}

macro_rules! BorrowDecoder {
    () => {
        deps!();
        # [doc = " Any source that can decode basic types. This type is most notably implemented for [Decoder]."] # [doc = ""] # [doc = " This is an extension of [Decode] that can also return borrowed data."] pub trait BorrowDecoder < 'de > : Decoder { # [doc = " The concrete [BorrowReader] type"] type BR : BorrowReader < 'de > ; # [doc = " Returns a mutable reference to the borrow reader"] fn borrow_reader (& mut self) -> & mut Self :: BR ; }
    };
}

BorrowDecoder!();