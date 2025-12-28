macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < Error > for io :: Error { fn from (err : Error) -> Self { match err . raw_os_error () { Some (errno) => io :: Error :: from_raw_os_error (errno) , None => io :: Error :: other (err) , } } }
    };
}

impl_22!();