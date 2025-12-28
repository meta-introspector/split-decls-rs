macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T : StdError + Send + Sync + 'static > From < Box < T > > for Error { fn from (value : Box < T >) -> Self { Error :: Other (value) } }
    };
}

impl_4!();