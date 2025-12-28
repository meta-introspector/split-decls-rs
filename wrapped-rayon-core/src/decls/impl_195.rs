macro_rules! deps {
    () => {
        ScopePtr!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T > ScopePtr < T > { unsafe fn as_ref (& self) -> & T { unsafe { & * self . 0 } } }
    };
}

impl_195!()