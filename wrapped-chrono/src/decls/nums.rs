macro_rules! deps {
    () => {
        Item!();
        Numeric!();
        Pad!();
    };
}

macro_rules! nums {
    () => {
        deps!();
        const fn nums (numeric : Numeric) -> Item < 'static > { Item :: Numeric (numeric , Pad :: Space) }
    };
}

nums!();