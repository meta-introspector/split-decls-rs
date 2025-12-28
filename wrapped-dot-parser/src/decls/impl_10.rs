macro_rules! deps {
    () => {
        IOError!();
        GraphFromFileError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl From < IOError > for GraphFromFileError < '_ > { fn from (e : IOError) -> Self { Self :: FileError (e) } }
    };
}

impl_10!()