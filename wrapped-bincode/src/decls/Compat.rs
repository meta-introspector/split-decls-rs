macro_rules! deps {
    () => {
        Encode!();
        Decode!();
        BorrowCompat!();
    };
}

macro_rules! Compat {
    () => {
        deps!();
        # [doc = " Wrapper struct that implements [Decode] and [Encode] on any type that implements serde's [DeserializeOwned] and [Serialize] respectively."] # [doc = ""] # [doc = " This works for most types, but if you're dealing with borrowed data consider using [BorrowCompat] instead."] # [doc = ""] # [doc = " [Decode]: ../de/trait.Decode.html"] # [doc = " [Encode]: ../enc/trait.Encode.html"] # [doc = " [DeserializeOwned]: https://docs.rs/serde/1/serde/de/trait.DeserializeOwned.html"] # [doc = " [Serialize]: https://docs.rs/serde/1/serde/trait.Serialize.html"] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Default)] pub struct Compat < T > (pub T) ;
    };
}

Compat!();