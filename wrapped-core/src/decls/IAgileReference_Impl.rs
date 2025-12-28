macro_rules! deps {
    () => {
        GUID!();
        IUnknownImpl!();
    };
}

macro_rules! IAgileReference_Impl {
    () => {
        deps!();
        pub trait IAgileReference_Impl : windows_core :: IUnknownImpl { fn Resolve (& self , riid : * const windows_core :: GUID , ppvobjectreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: Result < () > ; }
    };
}

IAgileReference_Impl!();