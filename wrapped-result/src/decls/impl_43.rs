macro_rules! deps {
    () => {
        GUID!();
        IUnknown_Vtbl!();
        ComPtr!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl ComPtr { pub fn as_raw (& self) -> * mut core :: ffi :: c_void { unsafe { core :: mem :: transmute_copy (self) } } pub fn cast (& self , iid : & GUID) -> Option < Self > { let mut result = None ; unsafe { com_call ! (IUnknown_Vtbl , self . QueryInterface (iid , & mut result as * mut _ as _)) ; } result } }
    };
}

impl_43!();