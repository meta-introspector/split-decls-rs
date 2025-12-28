macro_rules! deps {
    () => {
        Error!();
        Data!();
    };
}

macro_rules! Find {
    () => {
        deps!();
        # [doc = " Find an object in the object store."] # [doc = ""] # [doc = " ## Notes"] # [doc = ""] # [doc = " Find effectively needs [generic associated types][issue] to allow a trait for the returned object type."] # [doc = " Until then, we will have to make due with explicit types and give them the potentially added features we want."] # [doc = ""] # [doc = " [issue]: https://github.com/rust-lang/rust/issues/44265"] pub trait Find { # [doc = " Find an object matching `id` in the database while placing its raw, possibly encoded data into `buffer`."] # [doc = ""] # [doc = " Returns `Some` object if it was present in the database, or the error that occurred during lookup or object"] # [doc = " retrieval."] fn try_find < 'a > (& self , id : & gix_hash :: oid , buffer : & 'a mut Vec < u8 >) -> Result < Option < crate :: Data < 'a > > , find :: Error > ; }
    };
}

Find!();