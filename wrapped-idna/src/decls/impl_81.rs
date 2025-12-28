macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl From < Errors > for Result < () , Errors > { fn from (e : Errors) -> Self { Err (e) } }
    };
}

impl_81!()