macro_rules! deps {
    () => {
        CvQualifiers!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl CvQualifiers { # [inline] fn is_empty (& self) -> bool { ! self . restrict && ! self . volatile && ! self . const_ } }
    };
}

impl_165!()