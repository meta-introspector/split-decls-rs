macro_rules! deps {
    () => {
        RwLock!();
        Action!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl From < rt :: rwlock :: Action > for Action { fn from (action : rt :: rwlock :: Action) -> Self { Action :: RwLock (action) } }
    };
}

impl_102!();