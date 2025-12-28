macro_rules! deps {
    () => {
        FromOsStrError!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl error :: Error for FromOsStrError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { None } }
    };
}

impl_118!()