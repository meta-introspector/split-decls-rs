macro_rules! deps {
    () => {
        Request!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T : Into < String > > From < T > for Request { fn from (query : T) -> Self { Self :: new (query) } }
    };
}

impl_100!()