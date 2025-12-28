macro_rules! deps {
    () => {
        CustomError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl From < u32 > for CustomError { fn from (_ : u32) -> Self { CustomError } }
    };
}

impl_73!()