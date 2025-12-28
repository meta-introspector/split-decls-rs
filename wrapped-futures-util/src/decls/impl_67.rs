macro_rules! impl_67 {
    () => {
        impl < Fut > CatchUnwind < Fut > where Fut : Future + UnwindSafe , { pub (super) fn new (future : Fut) -> Self { Self { future } } }
    };
}

impl_67!()