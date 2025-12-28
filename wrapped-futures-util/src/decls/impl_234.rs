macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < Fut > SelectAll < Fut > { # [doc = " Consumes this combinator, returning the underlying futures."] pub fn into_inner (self) -> Vec < Fut > { self . inner } }
    };
}

impl_234!()