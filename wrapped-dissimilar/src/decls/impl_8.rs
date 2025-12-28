macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl AsRef < [char] > for Range < '_ > { fn as_ref (& self) -> & [char] { slice (* self) } }
    };
}

impl_8!();