macro_rules! impl_26 {
    () => {
        impl < Fut > Fuse < Fut > { fn new (f : Fut) -> Self { Self { inner : Some (f) } } }
    };
}

impl_26!();