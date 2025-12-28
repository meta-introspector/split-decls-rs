macro_rules! deps {
    () => {
        EndLine!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl EndLine { # [doc = " The number of characters this line ending occupies in bytes."] pub (crate) fn len (self) -> usize { match self { EndLine :: Eof => 0 , EndLine :: Lf => 1 , EndLine :: Crlf => 2 , } } }
    };
}

impl_67!()