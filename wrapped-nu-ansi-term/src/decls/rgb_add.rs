macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! rgb_add {
    () => {
        deps!();
        const fn rgb_add (lhs : & Rgb , rhs : & Rgb) -> Rgb { Rgb :: new (lhs . r . saturating_add (rhs . r) , lhs . g . saturating_add (rhs . g) , lhs . b . saturating_add (rhs . b) ,) }
    };
}

rgb_add!()