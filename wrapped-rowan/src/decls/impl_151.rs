macro_rules! deps {
    () => {
        ThinArc!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        unsafe impl < H : Sync + Send , T : Sync + Send > Sync for ThinArc < H , T > { }
    };
}

impl_151!();