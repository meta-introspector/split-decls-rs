macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl From < & str > for Error { fn from (err : & str) -> Self { err . to_string () . into () } }
    };
}

impl_2!()