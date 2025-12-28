macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_854 {
    () => {
        deps!();
        unsafe impl < Fut : Sync + Unpin > Sync for IntoIter < Fut > { }
    };
}

impl_854!()