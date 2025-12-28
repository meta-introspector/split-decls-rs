macro_rules! deps {
    () => {
        Pad!();
        Item!();
        Numeric!();
    };
}

macro_rules! num0 {
    () => {
        deps!();
        const fn num0 (numeric : Numeric) -> Item < 'static > { Item :: Numeric (numeric , Pad :: Zero) }
    };
}

num0!();