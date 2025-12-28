macro_rules! deps {
    () => {
        ThinArc!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < H : Eq , T : Eq > Eq for ThinArc < H , T > { }
    };
}

impl_159!()