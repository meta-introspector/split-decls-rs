macro_rules! deps {
    () => {
        InitError!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl From < String > for InitError { fn from (message : String) -> Self { Self { message , span : None , nested : None } } }
    };
}

impl_100!();