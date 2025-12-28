macro_rules! deps {
    () => {
        NewTypeName!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl FromMeta for NewTypeName { fn from_word () -> darling :: Result < Self > { Ok (Self :: Rust) } fn from_string (value : & str) -> darling :: Result < Self > { Ok (Self :: New (value . to_string ())) } fn from_bool (value : bool) -> darling :: Result < Self > { if value { Ok (Self :: Rust) } else { Ok (Self :: Original) } } }
    };
}

impl_52!()