macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl From < & Header < '_ > > for BString { fn from (header : & Header < '_ >) -> Self { header . to_bstring () } }
    };
}

impl_166!()