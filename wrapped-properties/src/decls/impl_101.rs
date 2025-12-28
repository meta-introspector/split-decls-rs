macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl From < u32 > for GeneralCategoryGroup { fn from (mask : u32) -> Self { GeneralCategoryGroup (mask & Self :: ALL) } }
    };
}

impl_101!()