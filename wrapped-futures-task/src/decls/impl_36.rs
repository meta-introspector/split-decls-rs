macro_rules! deps {
    () => {
        LocalFutureObj!();
        FutureObj!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a , T > From < FutureObj < 'a , T > > for LocalFutureObj < 'a , T > { # [inline] fn from (f : FutureObj < 'a , T >) -> Self { f . 0 } }
    };
}

impl_36!()