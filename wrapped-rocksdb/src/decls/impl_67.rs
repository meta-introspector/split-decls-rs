macro_rules! deps {
    () => {
        Decision!();
        CompactionFilterFn!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < F > CompactionFilterFn for F where F : FnMut (u32 , & [u8] , & [u8]) -> Decision + Send + 'static { }
    };
}

impl_67!()