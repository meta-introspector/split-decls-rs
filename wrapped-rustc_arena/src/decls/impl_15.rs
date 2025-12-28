macro_rules! deps {
    () => {
        DroplessArena!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl Send for DroplessArena { }
    };
}

impl_15!();