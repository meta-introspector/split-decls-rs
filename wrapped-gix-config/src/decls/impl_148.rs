macro_rules! deps {
    () => {
        Comment!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl From < Comment < '_ > > for BString { fn from (c : Comment < '_ >) -> Self { c . to_bstring () } }
    };
}

impl_148!();