macro_rules! deps {
    () => {
        IntoStream!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < S : Stream > IntoStream for S { type Item = S :: Item ; type IntoStream = S ; # [inline] fn into_stream (self) -> S { self } }
    };
}

impl_432!();