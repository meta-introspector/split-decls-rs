macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl From < File < '_ > > for BString { fn from (c : File < '_ >) -> Self { c . to_bstring () } }
    };
}

impl_48!()