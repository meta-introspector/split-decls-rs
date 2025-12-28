macro_rules! deps {
    () => {
        Action!();
        Atomic!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl From < rt :: atomic :: Action > for Action { fn from (action : rt :: atomic :: Action) -> Self { Action :: Atomic (action) } }
    };
}

impl_100!()