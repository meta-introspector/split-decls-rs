macro_rules! deps {
    () => {
        InterfaceRef!();
        Param!();
        ParamValue!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T > Param < T > for InterfaceRef < '_ , T > where T : Type < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Borrowed (transmute_copy (& self)) } } }
    };
}

impl_177!();