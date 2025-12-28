macro_rules! deps {
    () => {
        PrepareCheckout!();
        Repository!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl From < PrepareCheckout > for Repository { fn from (prep : PrepareCheckout) -> Self { prep . persist () } }
    };
}

impl_99!()