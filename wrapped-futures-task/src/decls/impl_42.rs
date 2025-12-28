macro_rules! deps {
    () => {
        FutureObj!();
        UnsafeFutureObj!();
        LocalFutureObj!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'a , T > FutureObj < 'a , T > { # [doc = " Create a `FutureObj` from a custom trait object representation."] # [inline] pub fn new < F : UnsafeFutureObj < 'a , T > + Send > (f : F) -> Self { Self (LocalFutureObj :: new (f)) } }
    };
}

impl_42!();