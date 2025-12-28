macro_rules! deps {
    () => {
        Error!();
        MultiError!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < MultiError > for io :: Error { fn from (e : MultiError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
    };
}

impl_23!();