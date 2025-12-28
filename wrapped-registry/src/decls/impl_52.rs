macro_rules! deps {
    () => {
        SECURITY_ATTRIBUTES!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Default for SECURITY_ATTRIBUTES { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_52!();