macro_rules! impl_47 {
    () => {
        impl < Fut > Fuse < Fut > { pub (super) fn new (f : Fut) -> Self { Self { inner : Some (f) } } }
    };
}

impl_47!()