macro_rules! deps {
    () => {
        FilePathToUriError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl error :: Error for FilePathToUriError { }
    };
}

impl_8!();