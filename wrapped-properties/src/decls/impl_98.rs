macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl crate :: private :: Sealed for GeneralCategoryGroup { }
    };
}

impl_98!();