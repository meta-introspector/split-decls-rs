macro_rules! deps {
    () => {
        IIterator!();
        IIterator_Vtbl!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl < T : windows_core :: RuntimeType + 'static > windows_core :: Interface for IIterator < T > { type Vtable = IIterator_Vtbl < T > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_15!()