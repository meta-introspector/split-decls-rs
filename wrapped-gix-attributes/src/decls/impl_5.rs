macro_rules! deps {
    () => {
        Name!();
        NameRef!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl NameRef < '_ > { # [doc = " Turn this ref into its owned counterpart."] pub fn to_owned (self) -> Name { Name (self . 0 . into ()) } # [doc = " Return the inner `str`."] pub fn as_str (& self) -> & str { self . 0 . as_str () } }
    };
}

impl_5!();