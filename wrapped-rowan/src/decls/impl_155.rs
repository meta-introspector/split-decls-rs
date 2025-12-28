macro_rules! deps {
    () => {
        ThinArc!();
        Arc!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < H , T > Clone for ThinArc < H , T > { # [inline] fn clone (& self) -> Self { ThinArc :: with_arc (self , | a | Arc :: into_thin (a . clone ())) } }
    };
}

impl_155!();