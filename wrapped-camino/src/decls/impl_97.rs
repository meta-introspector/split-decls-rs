macro_rules! deps {
    () => {
        FromOsStringError!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl error :: Error for FromOsStringError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { Some (& self . error) } }
    };
}

impl_97!()