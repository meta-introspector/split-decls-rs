macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T : Clone > Default for Stack < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_165!()