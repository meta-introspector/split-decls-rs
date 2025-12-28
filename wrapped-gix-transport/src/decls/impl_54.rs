macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl From < Error > for http :: Error { fn from (err : Error) -> Self { http :: Error :: Detail { description : err . to_string () , } } }
    };
}

impl_54!()