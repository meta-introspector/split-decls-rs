macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl From < NulError > for Error { # [cold] fn from (err : NulError) -> Self { Self :: NulError (err) } }
    };
}

impl_3!()