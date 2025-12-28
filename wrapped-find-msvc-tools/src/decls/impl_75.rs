macro_rules! deps {
    () => {
        SAFEARRAY!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Default for SAFEARRAY { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_75!();