macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl < H : Send > Send for Inner < H > { }
    };
}

impl_62!();