macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl From < curl :: Error > for http :: Error { fn from (err : curl :: Error) -> Self { http :: Error :: Detail { description : err . to_string () , } } }
    };
}

impl_55!();