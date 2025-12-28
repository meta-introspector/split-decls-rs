macro_rules! deps {
    () => {
        FromPathBufError!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl error :: Error for FromPathBufError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { Some (& self . error) } }
    };
}

impl_89!()