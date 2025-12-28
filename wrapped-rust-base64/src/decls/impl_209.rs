macro_rules! deps {
    () => {
        ParseAlphabetError!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , test))] impl error :: Error for ParseAlphabetError { }
    };
}

impl_209!();