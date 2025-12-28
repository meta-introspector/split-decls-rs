macro_rules! deps {
    () => {
        TypedArena!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for TypedArena < T > { }
    };
}

impl_10!();