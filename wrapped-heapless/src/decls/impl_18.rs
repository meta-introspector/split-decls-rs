macro_rules! deps {
    () => {
        ExtendError!();
        CapacityError!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl From < CapacityError > for ExtendError { fn from (error : CapacityError) -> Self { Self :: Capacity (error) } }
    };
}

impl_18!()