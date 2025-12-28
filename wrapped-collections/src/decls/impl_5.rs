macro_rules! deps {
    () => {
        IIterator!();
        IIterable!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > IIterable < T > { pub fn First (& self) -> windows_core :: Result < IIterator < T > > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . First) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
    };
}

impl_5!()