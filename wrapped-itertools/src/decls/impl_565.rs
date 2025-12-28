macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        impl Position { # [doc = " This is the first and the last element at the same time, and there are no more elements"] pub fn is_exactly_one (self) -> bool { self . is_first && self . is_last } # [doc = " This is neither first nor last element, and there will be more elements"] pub fn is_middle (self) -> bool { ! self . is_first && ! self . is_last } # [doc = " This is the initial element (also true if there's exactly one element)"] pub fn is_first (self) -> bool { self . is_first } # [doc = " This is the final element (also true if there's exactly one element)"] pub fn is_last (self) -> bool { self . is_last } }
    };
}

impl_565!()