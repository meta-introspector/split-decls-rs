macro_rules! deps {
    () => {
        Action!();
        Arc!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl From < rt :: arc :: Action > for Action { fn from (action : rt :: arc :: Action) -> Self { Action :: Arc (action) } }
    };
}

impl_99!()