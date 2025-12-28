macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl From < GeneralCategory > for GeneralCategoryGroup { fn from (subcategory : GeneralCategory) -> Self { GeneralCategoryGroup (1 << (subcategory as u32)) } }
    };
}

impl_100!();