macro_rules! deps {
    () => {
        ServerError!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl From < ServerError > for Vec < ServerError > { fn from (single : ServerError) -> Self { vec ! [single] } }
    };
}

impl_33!()