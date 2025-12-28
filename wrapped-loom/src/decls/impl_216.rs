macro_rules! deps {
    () => {
        UnsafeCell!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < T : Default > Default for UnsafeCell < T > { fn default () -> UnsafeCell < T > { UnsafeCell :: new (Default :: default ()) } }
    };
}

impl_216!();