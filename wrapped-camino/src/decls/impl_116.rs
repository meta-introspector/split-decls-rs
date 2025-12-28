macro_rules! deps {
    () => {
        FromOsStrError!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl FromOsStrError { # [doc = " Converts self into a [`std::io::Error`] with kind"] # [doc = " [`InvalidData`](io::ErrorKind::InvalidData)."] # [doc = ""] # [doc = " Many users of [`FromOsStrError`] will want to convert it into an [`io::Error`]. This is a"] # [doc = " convenience method to do that."] pub fn into_io_error (self) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , self) } }
    };
}

impl_116!();