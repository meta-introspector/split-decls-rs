macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl From < JoinPathsError > for Error { fn from (e : JoinPathsError) -> Error { Error :: from_str (& e . to_string ()) } }
    };
}

impl_377!();