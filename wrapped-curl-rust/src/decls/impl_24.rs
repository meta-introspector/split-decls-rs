macro_rules! deps {
    () => {
        FormError!();
        Error!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < FormError > for io :: Error { fn from (e : FormError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
    };
}

impl_24!()