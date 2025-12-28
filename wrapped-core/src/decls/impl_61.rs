macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl IWeakReferenceSource { pub unsafe fn GetWeakReference (& self) -> windows_core :: Result < IWeakReference > { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . GetWeakReference) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
    };
}

impl_61!();