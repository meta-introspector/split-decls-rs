macro_rules! deps {
    () => {
        Limit!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl From < usize > for Limit { fn from (value : usize) -> Self { Self :: new (value) } }
    };
}

impl_426!();