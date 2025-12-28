macro_rules! deps {
    () => {
        Error!();
        Kind!();
        Parse!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        # [doc (hidden)] impl From < Parse > for Error { fn from (err : Parse) -> Error { Error :: new (Kind :: Parse (err)) } }
    };
}

impl_113!()