macro_rules! deps {
    () => {
        FromPathError!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl error :: Error for FromPathError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { None } }
    };
}

impl_110!()