macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl From < & Event < '_ > > for BString { fn from (event : & Event < '_ >) -> Self { event . to_bstring () } }
    };
}

impl_143!();