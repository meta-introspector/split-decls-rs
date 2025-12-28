macro_rules! deps {
    () => {
        Documentation!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < Documentation > for String { fn from (Documentation (string) : Documentation) -> Self { string } }
    };
}

impl_51!()