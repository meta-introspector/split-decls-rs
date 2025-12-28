macro_rules! deps {
    () => {
        SECURITY_ATTRIBUTES!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Default for SECURITY_ATTRIBUTES { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_113!()