macro_rules! deps {
    () => {
        ThinArc!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        unsafe impl < H : Sync + Send , T : Sync + Send > Send for ThinArc < H , T > { }
    };
}

impl_150!()