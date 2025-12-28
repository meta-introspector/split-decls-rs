macro_rules! deps {
    () => {
        ShareError!();
        Error!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < ShareError > for io :: Error { fn from (e : ShareError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
    };
}

impl_22!();