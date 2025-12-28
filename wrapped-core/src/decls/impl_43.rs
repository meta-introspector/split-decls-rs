macro_rules! deps {
    () => {
        IAgileObject_Vtbl!();
        Interface!();
        IUnknown_Vtbl!();
        GUID!();
        IAgileObject_Impl!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl IAgileObject_Vtbl { pub const fn new < Identity : IAgileObject_Impl , const OFFSET : isize > () -> Self { Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAgileObject as windows_core :: Interface > :: IID } }
    };
}

impl_43!();