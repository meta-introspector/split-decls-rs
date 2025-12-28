macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < Errors > for Result < () , Errors > { fn from (e : Errors) -> Self { Err (e) } }
    };
}

impl_8!()