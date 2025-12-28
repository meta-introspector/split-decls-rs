macro_rules! deps {
    () => {
        Numeric!();
        Pad!();
        Item!();
    };
}

macro_rules! nums {
    () => {
        deps!();
        const fn nums (numeric : Numeric) -> Item < 'static > { Item :: Numeric (numeric , Pad :: Space) }
    };
}

nums!()