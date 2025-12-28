macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl From < f32 > for Value { # [inline] fn from (f : f32) -> Self { Self :: Real (f . into ()) } }
    };
}

impl_493!()