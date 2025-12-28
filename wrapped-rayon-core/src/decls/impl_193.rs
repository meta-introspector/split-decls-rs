macro_rules! deps {
    () => {
        ScopePtr!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        unsafe impl < T : Sync > Send for ScopePtr < T > { }
    };
}

impl_193!();