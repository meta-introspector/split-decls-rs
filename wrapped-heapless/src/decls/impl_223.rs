macro_rules! deps {
    () => {
        CapacityError!();
        FromUtf16Error!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl From < CapacityError > for FromUtf16Error { fn from (e : CapacityError) -> Self { Self :: Capacity (e) } }
    };
}

impl_223!()