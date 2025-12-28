macro_rules! deps {
    () => {
        IdentIsRaw!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl From < bool > for IdentIsRaw { fn from (b : bool) -> Self { if b { Self :: Yes } else { Self :: No } } }
    };
}

impl_407!()