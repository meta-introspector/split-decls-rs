macro_rules! deps {
    () => {
        IntoIter!();
        Send!();
    };
}

macro_rules! impl_853 {
    () => {
        deps!();
        unsafe impl < Fut : Send + Unpin > Send for IntoIter < Fut > { }
    };
}

impl_853!()