macro_rules! deps {
    () => {
        Numeric!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Numeric for bool { fn into_u64 (self) -> u64 { if self { 1 } else { 0 } } fn from_u64 (src : u64) -> bool { src != 0 } }
    };
}

impl_77!()