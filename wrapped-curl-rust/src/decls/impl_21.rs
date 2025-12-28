macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < Error > for io :: Error { fn from (e : Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
    };
}

impl_21!();