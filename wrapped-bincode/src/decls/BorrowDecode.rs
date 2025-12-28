macro_rules! deps {
    () => {
        Decode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! BorrowDecode {
    () => {
        deps!();
        # [doc = " Trait that makes a type able to be decoded, akin to serde's `Deserialize` trait."] # [doc = ""] # [doc = " This trait should be implemented for types that contain borrowed data, like `&str` and `&[u8]`. If your type does not have borrowed data, consider implementing [Decode] instead."] # [doc = ""] # [doc = " This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to a type with a lifetime."] pub trait BorrowDecode < 'de , Context > : Sized { # [doc = " Attempt to decode this type with the given [BorrowDecode]."] fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > ; }
    };
}

BorrowDecode!()