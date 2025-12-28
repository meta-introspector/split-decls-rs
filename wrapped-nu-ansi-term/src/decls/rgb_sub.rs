macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! rgb_sub {
    () => {
        deps!();
        const fn rgb_sub (lhs : & Rgb , rhs : & Rgb) -> Rgb { Rgb :: new (lhs . r . saturating_sub (rhs . r) , lhs . g . saturating_sub (rhs . g) , lhs . b . saturating_sub (rhs . b) ,) }
    };
}

rgb_sub!()