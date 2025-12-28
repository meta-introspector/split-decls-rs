macro_rules! deps {
    () => {
        WeakKeyError!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl core :: error :: Error for WeakKeyError { }
    };
}

impl_35!()