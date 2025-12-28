macro_rules! deps {
    () => {
        HybridProtection!();
        RefCnt!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T : RefCnt > Borrow < T > for HybridProtection < T > { # [inline] fn borrow (& self) -> & T { & self . ptr } }
    };
}

impl_108!()