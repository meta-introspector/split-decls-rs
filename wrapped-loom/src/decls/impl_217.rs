macro_rules! deps {
    () => {
        UnsafeCell!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < T > From < T > for UnsafeCell < T > { fn from (src : T) -> UnsafeCell < T > { UnsafeCell :: new (src) } }
    };
}

impl_217!();