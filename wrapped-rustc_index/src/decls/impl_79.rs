macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Idx for usize { # [inline] fn new (idx : usize) -> Self { idx } # [inline] fn index (self) -> usize { self } }
    };
}

impl_79!();