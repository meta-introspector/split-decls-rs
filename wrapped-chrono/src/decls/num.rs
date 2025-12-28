macro_rules! deps {
    () => {
        Pad!();
        Item!();
        Numeric!();
    };
}

macro_rules! num {
    () => {
        deps!();
        const fn num (numeric : Numeric) -> Item < 'static > { Item :: Numeric (numeric , Pad :: None) }
    };
}

num!();