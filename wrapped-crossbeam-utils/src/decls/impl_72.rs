macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for CachePadded < T > { }
    };
}

impl_72!()