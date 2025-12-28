macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl From < GeneralCategoryGroup > for u32 { fn from (group : GeneralCategoryGroup) -> Self { group . 0 } }
    };
}

impl_102!()