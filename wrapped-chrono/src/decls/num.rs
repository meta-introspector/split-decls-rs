macro_rules! deps {
    () => {
        Numeric!();
        Item!();
        Pad!();
    };
}

macro_rules! num {
    () => {
        deps!();
        const fn num (numeric : Numeric) -> Item < 'static > { Item :: Numeric (numeric , Pad :: None) }
    };
}

num!()