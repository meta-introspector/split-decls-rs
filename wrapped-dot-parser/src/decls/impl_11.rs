macro_rules! deps {
    () => {
        PestError!();
        GraphFromFileError!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < PestError > for GraphFromFileError < '_ > { fn from (e : PestError) -> Self { Self :: PestParseError (e) } }
    };
}

impl_11!();