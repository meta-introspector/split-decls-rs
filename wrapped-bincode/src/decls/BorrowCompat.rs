macro_rules! deps {
    () => {
        BorrowDecode!();
        Compat!();
        Encode!();
    };
}

macro_rules! BorrowCompat {
    () => {
        deps!();
        # [doc = " Wrapper struct that implements [BorrowDecode] and [Encode] on any type that implements serde's [Deserialize] and [Serialize] respectively. This is mostly used on `&[u8]` and `&str`, for other types consider using [Compat] instead."] # [doc = ""] # [doc = " [BorrowDecode]: ../de/trait.BorrowDecode.html"] # [doc = " [Encode]: ../enc/trait.Encode.html"] # [doc = " [Deserialize]: https://docs.rs/serde/1/serde/de/trait.Deserialize.html"] # [doc = " [Serialize]: https://docs.rs/serde/1/serde/trait.Serialize.html"] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Default)] pub struct BorrowCompat < T > (pub T) ;
    };
}

BorrowCompat!();