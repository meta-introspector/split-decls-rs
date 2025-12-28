macro_rules! deps {
    () => {
        Json!();
    };
}

macro_rules! impl_777 {
    () => {
        deps!();
        impl < T > From < T > for Json < T > { fn from (value : T) -> Self { Self (value) } }
    };
}

impl_777!()