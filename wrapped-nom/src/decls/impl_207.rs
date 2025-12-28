macro_rules! deps {
    () => {
        ErrorKind!();
        NilError!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < I > From < (I , ErrorKind) > for NilError { fn from (_ : (I , ErrorKind)) -> Self { NilError } }
    };
}

impl_207!()