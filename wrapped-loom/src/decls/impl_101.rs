macro_rules! deps {
    () => {
        Channel!();
        Action!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl From < rt :: mpsc :: Action > for Action { fn from (action : rt :: mpsc :: Action) -> Self { Action :: Channel (action) } }
    };
}

impl_101!();