macro_rules! deps {
    () => {
        GraphFromFileError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Error for GraphFromFileError < '_ > { }
    };
}

impl_9!();