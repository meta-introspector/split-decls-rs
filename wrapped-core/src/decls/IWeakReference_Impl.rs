macro_rules! deps {
    () => {
        GUID!();
        IUnknownImpl!();
    };
}

macro_rules! IWeakReference_Impl {
    () => {
        deps!();
        pub trait IWeakReference_Impl : windows_core :: IUnknownImpl { fn Resolve (& self , riid : * const windows_core :: GUID , objectreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: Result < () > ; }
    };
}

IWeakReference_Impl!();