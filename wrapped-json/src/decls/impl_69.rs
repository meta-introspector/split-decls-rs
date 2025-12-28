macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl ser :: Error for Error { # [cold] fn custom < T : Display > (msg : T) -> Error { make_error (msg . to_string ()) } }
    };
}

impl_69!()